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

    let seed = 0;
    let perlin = Perlin::new(seed);
    let (mut map, noise_map) = map::generate_map(seed, WIDTH, HEIGHT);

    let mut robots = vec![
        Robot::new(WIDTH as i32, HEIGHT as i32, &map),
        Robot::new(WIDTH as i32, HEIGHT as i32, &map),
        Robot::new(WIDTH as i32, HEIGHT as i32, &map),
    ];

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut base = Base::new();

    loop {
        let frame_start = Instant::now();

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
            robot.discover_current_location(biome, resource);
            robot.update(&mut map);
        }

        let mut grid = map.render();
        for robot in &robots {
            robot.render(&mut grid);
        }
        let lines: Vec<Line> = grid.iter().map(|row| Line::from(row.clone())).collect();

        terminal.draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());
            let map_widget = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(map_widget, layout[0]);
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