use std::{io, thread, time::{Duration, Instant}, sync::{Arc, Mutex}};
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
    const NUM_ROBOTS: usize = 3;  // Nombre de robots à créer

    let seed = 155;
    let perlin = Arc::new(Perlin::new(seed));
    let (init_map, noise_map) = map::generate_map(seed, WIDTH, HEIGHT);
    
    // Partager les ressources entre les threads
    let map = Arc::new(Mutex::new(init_map));
    let base = Arc::new(Mutex::new(Base::new(WIDTH, HEIGHT)));
    
    // Utiliser Arc<Mutex<Vec<Robot>>> pour partager les robots entre les threads
    let robots = Arc::new(Mutex::new(Vec::new()));
    
    // Créer des robots dans des threads séparés
    let mut robot_handles = Vec::new();
    
    for i in 0..NUM_ROBOTS {
        let robot_map = Arc::clone(&map);
        let robot_base = Arc::clone(&base);
        let robot_perlin = Arc::clone(&perlin);
        let robots_clone = Arc::clone(&robots);
        let i_clone = i;
        
        let mut robot = Robot::new(WIDTH, HEIGHT, &map.lock().unwrap());
        if i % 2 == 0 {
            robot.class = Some("scientist".to_string());
        } else {
            robot.class = Some("miner".to_string());
        }
        
        robot.id = i as i32;
        
        robots.lock().unwrap().push(robot.clone());
        
        let handle = thread::spawn(move || {
            let mut robot_thread = robot;
            let mut previous_time = Instant::now();
            
            loop {
                let current_time = Instant::now();
                let delta_time = current_time.duration_since(previous_time);
                previous_time = current_time;
                
                let mut map_guard = robot_map.lock().unwrap();
                let mut base_guard = robot_base.lock().unwrap();
                
                let noise_at_robot = robot_perlin.get([robot_thread.x as f64 / 10.0, robot_thread.y as f64 / 10.0, 0.0]);
                let biome = get_biome_from_noise(noise_at_robot);
                let resource = get_resource_from_biome(noise_at_robot, biome);
                
                if let Some(tile) = map_guard.blueprint
                    .get_mut(robot_thread.x as usize)
                    .and_then(|row| row.get_mut(robot_thread.y as usize)) {
                    tile.biome = biome;
                    tile.resource = resource;
                }
                
                robot_thread.discover_current_location(biome, resource);
                robot_thread.update(&mut map_guard, &mut base_guard, delta_time);
                
                if let Some(tile) = map_guard.blueprint
                    .get(robot_thread.x as usize)
                    .and_then(|row| row.get(robot_thread.y as usize)) {
                    if tile.resource == Resource::Iron {
                        robot_thread.iron_collected += 1;
                        robot_thread.collect_iron(&mut map_guard);
                    } else if tile.resource == Resource::Research {
                        robot_thread.research_collected += 1;
                        robot_thread.collect_research(&mut map_guard);
                    }
                }
                
                drop(map_guard);
                drop(base_guard);
                
                let mut robots_guard = robots_clone.lock().unwrap();
                robots_guard[i_clone] = robot_thread.clone();
                drop(robots_guard);
                
                thread::sleep(Duration::from_millis(100));
            }
        });
        
        robot_handles.push(handle);
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let session_start = Instant::now();
    let mut previous_frame = Instant::now();

    loop { // Gameloop
        let frame_start = Instant::now();
        let delta_time = frame_start.duration_since(previous_frame);
        previous_frame = frame_start;

        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('u') => {
                        let mut base_guard = base.lock().unwrap();
                        base_guard.upgrade_base();
                    },
                    _ => {}
                }
            }
        }

        {
            let mut base_guard = base.lock().unwrap();
            base_guard.generate_energy();
        }

        let map_guard = map.lock().unwrap();
        let base_guard = base.lock().unwrap();
        let robots_guard = robots.lock().unwrap();

        let mut grid = map_guard.render();
        grid[base_guard.y as usize][base_guard.x as usize] = Span::styled("B", Style::default().fg(Color::Green));
        
        for robot in robots_guard.iter() {
            robot.render(&mut grid);
        }
        
        let base_info = format!(
            "Base Info:\nPosition: ({}, {})\nEnergy: {} / {}\nIron: {}\nResearch: {}\nBase lvl: {}\nAppuyez sur 'u' pour améliorer la base.",
            base_guard.x, base_guard.y, base_guard.energy, base_guard.energy_capacity, 
            base_guard.iron, base_guard.research, base_guard.lvl
        );
        
        let session_time = session_start.elapsed();
        let debug_info = format!("Session Time: {:.2?} | Robots: {}", session_time, robots_guard.len());
        
        let robot_info = if !robots_guard.is_empty() {
            let robot = &robots_guard[0];
            format!(
                "Robot Info:\nPosition: ({}, {})\nEnergy: {}\nIron: {}\nResearch: {}\nClass: {}\n{}",
                robot.x, robot.y, robot.energy, robot.iron_collected, robot.research_collected, 
                robot.class.as_deref().unwrap_or("None"), debug_info
            )
        } else {
            format!("No robots available\n{}", debug_info)
        };
        
        let lines: Vec<Line> = grid.iter().map(|row| Line::from(row.clone())).collect();
        
        drop(base_guard);
        drop(robots_guard);

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());
            let map_widget = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title("Map"));
            f.render_widget(map_widget, chunks[0]);
        
            let info_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);
            let robot_info_widget = Paragraph::new(robot_info)
                .block(Block::default().borders(Borders::ALL).title("Robots Info"));
            f.render_widget(robot_info_widget, info_chunks[0]);
        
            let base_info_widget = Paragraph::new(base_info)
                .block(Block::default().borders(Borders::ALL).title("Base Info"));
            f.render_widget(base_info_widget, info_chunks[1]);
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