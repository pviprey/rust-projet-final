use noise::{NoiseFn, Perlin};
use ratatui::text::Span;
use ratatui::style::{Style, Color};

#[derive(Copy, Clone)]
pub enum Biome {
    Plain,
    Desert,
    Forest,
    Mountain,
    Water,
}

#[derive(Copy, Clone)]
pub enum Resource {
    None,
    Iron,
    Research,
}

#[derive(Clone)]
pub struct TileInfo {
    pub biome: Biome,
    pub resource: Resource,
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
                    Resource::Iron => {
                        let ch = if *collected { "." } else { "F" };
                        Span::styled(ch, Style::default().fg(Color::Yellow))
                    }
                    Resource::Research => Span::styled("T", Style::default().fg(Color::Cyan)),
                    Biome::Mountain => Span::styled("#", Style::default().fg(Color::DarkGray)),
                    Biome::Plain => Span::raw("."),
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
                Biome::Mountain
            } else if noise < -0.35 {
                Resource::Iron
            } else if noise < -0.3 {
                Resource::Research
            } else {
                Biome::Plain
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

pub fn get_biome_from_noise(noise: f64) -> Biome {
    if noise < -0.4 {
        Biome::Mountain
    } else {
        Biome::Plain
    }
}

pub fn get_resource_from_noise(noise: f64) -> Resource {
    if noise < -0.35 && noise >= -0.4 {
        Resource::Iron
    } else if noise < -0.3 && noise >= -0.35 {
        Resource::Research
    } else {
        Resource::None
    }
}