use noise::{NoiseFn, Perlin};
use ratatui::text::Span;
use ratatui::style::{Style, Color};

#[derive(Clone)]
pub enum Terrain {
    Iron { collected: bool },
    Research,
    Mountain,
    Plain,
}

#[derive(Clone)]
pub struct Map {
    pub seed: u32,
    pub width: i32,
    pub height: i32,
    pub blueprint: Vec<Vec<Terrain>>,
}

impl Map {
    pub fn render(&self) -> Vec<Vec<Span>> {
        let mut grid = vec![vec![Span::raw(" "); self.width as usize]; self.height as usize];

        for y in 0..self.height {
            for x in 0..self.width {
                let span = match &self.blueprint[x as usize][y as usize] {
                    Terrain::Iron { collected } => {
                        let ch = if *collected { "." } else { "F" };
                        Span::styled(ch, Style::default().fg(Color::Yellow))
                    }
                    Terrain::Research => Span::styled("T", Style::default().fg(Color::Cyan)),
                    Terrain::Mountain => Span::styled("#", Style::default().fg(Color::DarkGray)),
                    Terrain::Plain => Span::raw("."),
                };
                grid[y as usize][x as usize] = span;
            }
        }

        grid
    }
}

pub fn generate_map(seed: u32, width: i32, height: i32) -> Map {
    let perlin = Perlin::new(seed);
    let mut blueprint = vec![];

    for x in 0..width {
        let mut row = vec![];
        for y in 0..height {
            let noise = perlin.get([x as f64 / 10.0, y as f64 / 10.0, 0.0]);
            let terrain = if noise < -0.4 {
                Terrain::Mountain
            } else if noise < -0.35 {
                Terrain::Iron { collected: false }
            } else if noise < -0.3 {
                Terrain::Research
            } else {
                Terrain::Plain
            };
            row.push(terrain);
        }
        blueprint.push(row);
    }

    Map {
        seed,
        width,
        height,
        blueprint,
    }
}







/*

pub fn generate_map_with_robot() {
    let seed = 0;
    let perlin = Perlin::new(seed);
    let map_width = 20;
    let map_height = 100;
    let mut robot = Robot::new(map_width, map_height);
    
    let mut iron_collected = vec![vec![false; map_height]; map_width];
    let mut research_collected = vec![vec![false; map_height]; map_width];
    
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        
        let noise_at_robot = perlin.get([robot.x as f64 / 10.0, robot.y as f64 / 10.0, 0.0]);
        if noise_at_robot < -0.1 && noise_at_robot >= -0.3 && !iron_collected[robot.x][robot.y] {
            robot.collect_iron();
            iron_collected[robot.x][robot.y] = true;
        } else if noise_at_robot < -0.3 && !research_collected[robot.x][robot.y] {
            robot.collect_research();
        }
        
        // status
        println!("Iron Collected: {}", robot.iron_collected);
        println!("Research Collected: {}", robot.research_collected);
        println!("Energy: {}", robot.energy);
        println!();
        
        // map
        for x in 0..map_width {
            for y in 0..map_height {
                if robot.x == x && robot.y == y {
                    print!("R");
                } else {
                    let noise = perlin.get([x as f64 / 10.0, y as f64 / 10.0, 0.0]);
                    if noise < -0.4 {
                        print!("#");
                    } else if noise < -0.35 {
                        if !iron_collected[x][y] {
                            print!("F");
                        } else {
                            print!(".");
                        }
                    } else if noise < -0.3 {
                        print!("T");
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
        
        robot.move_random();
        thread::sleep(Duration::from_millis(200));
    }
}
    */