use std::{io, thread, time::{Duration, Instant}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    text::Line,
    style::{Style, Color},
    text::Span,
};
use noise::{NoiseFn, Perlin};
use maps::map::{self, Map, get_biome_from_noise, get_resource_from_biome, Biome, Resource};
use robots::robot::{self, Robot};
use base::base::Base;

mod maps;
mod robots;
mod base;

fn main() -> Result<(), io::Error> {
    // Configuration générale
    const WIDTH: i32 = 150;
    const HEIGHT: i32 = 40;
    const FPS: u64 = 30;
    const FRAME_DURATION: Duration = Duration::from_millis(1000 / FPS);

    let seed = 155;
    let perlin = Perlin::new(seed);
    let (mut map, noise_map) = map::generate_map(seed, WIDTH, HEIGHT);

    let mut robots = vec![
        {
            let mut r = Robot::new(WIDTH as i32, HEIGHT as i32, &map);
            r.modules = Some(String::from("none")); // wheels, buoy, tracks
            r
        }
    ];

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut base = Base::new(WIDTH, HEIGHT);

    let session_start = Instant::now();
    let mut previous_frame = Instant::now();

    loop {
        let frame_start = Instant::now();

        let delta_time = frame_start.duration_since(previous_frame);
        previous_frame = frame_start;

        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }


        base.generate_energy();

        for robot in robots.iter_mut() {
            let noise_at_robot = perlin.get([robot.x as f64 / 10.0, robot.y as f64 / 10.0, 0.0]);
            let biome = get_biome_from_noise(noise_at_robot);
            let resource = get_resource_from_biome(noise_at_robot, biome);

            if let Some(tile) = map.blueprint
            .get_mut(robot.x as usize)
            .and_then(|row| row.get_mut(robot.y as usize))
        {
            tile.biome = biome;
            tile.resource = resource;
        }

            robot.discover_current_location(biome, resource);
            robot.update(&mut map, &mut base, delta_time);
        }

        let mut grid = map.render();
        grid[base.y as usize][base.x as usize] = Span::styled("B", Style::default().fg(Color::Green));
        for robot in &robots {
            robot.render(&mut grid);
        }
        let lines: Vec<Line> = grid.iter().map(|row| Line::from(row.clone())).collect();

        let session_time = session_start.elapsed();
        let debug_info = format!("Session Time: {:.2?}", session_time);
        
        let robot = &robots[0];
        let robot_info = format!(
            "Robot Starting Info:\nPosition: ({}, {})\nEnergy: {}\nIron Collected: {}\nResearch Collected: {}\n{}",
            robot.x, robot.y, robot.energy, robot.iron_collected, robot.research_collected, debug_info
        );

        terminal.draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());
            let map_widget = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title("Map"));
            f.render_widget(map_widget, layout[0]);
            let info_widget = Paragraph::new(robot_info)
                .block(Block::default().borders(Borders::ALL).title("Robot Info"));
            f.render_widget(info_widget, layout[1]);
        })?;

        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - elapsed); 
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}