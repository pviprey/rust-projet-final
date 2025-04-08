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
    pub blueprint: Vec<Vec<TileInfo>>,
}

impl Map {
    pub fn render(&self) -> Vec<Vec<Span>> {
        let width_usize = self.width as usize;
        let height_usize = self.height as usize;
        let mut grid = vec![vec![Span::raw(" "); width_usize]; height_usize];
    
        for y in 0..height_usize {
            for x in 0..width_usize {
                let tile = &self.blueprint[x][y];
                let span = match (tile.resource, tile.biome) {
                    (Resource::Iron, _) => {
                        Span::styled("F", Style::default().fg(Color::Yellow))
                    },
                    (Resource::Research, _) => {
                        Span::styled("T", Style::default().fg(Color::Cyan))
                    },
                    (_, Biome::Mountain) => {
                        Span::styled("#", Style::default().fg(Color::DarkGray))
                    },
                    _ => Span::raw("."),
                };
                grid[y][x] = span;
            }
        }
    
        grid
    }

    pub fn discover_area(&mut self, x: usize, y: usize, radius: usize, biome: Biome, resource: Resource) {
        let width = self.blueprint.len();
        let height = if width > 0 { self.blueprint[0].len() } else { 0 };
        let start_x = if x >= radius { x - radius } else { 0 };
        let start_y = if y >= radius { y - radius } else { 0 };
        let end_x = usize::min(x + radius, width - 1);
        let end_y = usize::min(y + radius, height - 1);
        for i in start_x..=end_x {
            for j in start_y..=end_y {
                self.blueprint[i][j] = TileInfo { biome, resource };
            }
        }
    }
}

pub fn generate_map(seed: u32, width: i32, height: i32) -> (Map, Vec<Vec<f64>>) {
    let perlin = Perlin::new(seed);
    let width_usize = width as usize;
    let height_usize = height as usize;
    let mut blueprint: Vec<Vec<TileInfo>> = vec![
        vec![TileInfo { biome: Biome::Plain, resource: Resource::None }; height_usize];
        width_usize
    ];
    let mut noise_map: Vec<Vec<f64>> = vec![vec![0.0; height_usize]; width_usize];

    for x in 0..width_usize {
        for y in 0..height_usize {
            let nx = x as f64 / width as f64;
            let ny = y as f64 / height as f64;
            let noise = perlin.get([nx * 10.0, ny * 10.0, 0.0]);
            noise_map[x][y] = noise;
            let biome = get_biome_from_noise(noise);
            let resource = get_resource_from_biome(noise, biome);
            blueprint[x][y] = TileInfo { biome, resource };
        }
    }

    let map = Map {
        seed,
        width,
        height,
        blueprint,
    };

    (map, noise_map)
}

pub fn get_biome_from_noise(noise: f64) -> Biome {
    if noise < -0.3 {
        Biome::Water
    } else if noise < -0.1 {
        Biome::Desert
    } else if noise < 0.1 {
        Biome::Plain
    } else if noise < 0.3 {
        Biome::Forest
    } else {
        Biome::Mountain
    }
}

pub fn get_resource_from_biome(noise: f64, _biome: Biome) -> Resource {
    if noise > -0.25 && noise < -0.23 {
        Resource::Iron
    } else if noise > 0.20 && noise < 0.23 {
        Resource::Research
    } else {
        Resource::None
    }
}