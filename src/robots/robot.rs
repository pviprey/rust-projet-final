use pathfinding::prelude::astar;
use crate::maps::map::{Map, Terrain};
use ratatui::style::{Style, Color, Modifier};
use ratatui::text::Span;

#[derive(Clone)]
pub struct Robot {
    pub x: i32,
    pub y: i32,
    pub energy: i32,
    pub iron_collected: i32,
    pub research_collected: i32,
    pub known_map: Map,
    map_width: i32,
    map_height: i32,

}

impl Robot {
    pub fn new(map_width: i32, map_height: i32, full_map: &Map) -> Self {
        Self {
            x: map_width / 2,
            y: map_height / 2,
            map_width,
            map_height,
            energy: 100,
            iron_collected: 0,
            research_collected: 0,
            known_map: full_map.clone(),
        }
    }

    pub fn collect_iron(&mut self) {
        self.iron_collected += 1;
    }

    pub fn collect_research(&mut self) {
        self.research_collected += 1;
    }

    pub fn moving(&mut self, deplacement: Option<(Vec<(i32, i32)>, u32)>) {
        if let Some((path, _)) = deplacement {
            for (x, y) in path {
                if self.energy <= 0 {
                    break;
                }
                self.x = x;
                self.y = y;
                self.energy -= 1;
            }
        }
    }

    pub fn update(&mut self, map: &Map) -> bool {
        if self.energy <= 0 {
            return false;
        }
    
        // Chercher la case Iron la plus proche
        let mut closest: Option<(Vec<(i32, i32)>, u32)> = None;
        for (x, row) in map.blueprint.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if let Terrain::Iron { collected: false } = tile {
                    let path = self.path_finding(x as i32, y as i32);
                    if let Some(p) = path {
                        if closest.is_none() || p.1 < closest.as_ref().unwrap().1 {
                            closest = Some(p);
                        }
                    }
                }
            }
        }
    
        self.moving(closest);
        true
    }
    

    pub fn path_finding(&mut self, dest_x: i32, dest_y: i32) -> Option<(Vec<(i32, i32)>, u32)> {
        let start = (self.x, self.y);
        let goal = (dest_x, dest_y);

        astar(
            &start,
            |&(x, y)| {
                let deltas: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                deltas
                    .iter()
                    .map(|&(dx, dy)| ((x + dx, y + dy), 1))
                    .collect::<Vec<_>>()
            },
            |&(x, y)| goal.0.abs_diff(x) + goal.1.abs_diff(y),
            |&p| p == goal,
        )
    }

    pub fn render(&self, grid: &mut Vec<Vec<Span>>) {
        let x = self.x as usize;
        let y = self.y as usize;

        grid[y][x] = Span::styled(
            "R",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        );
    }
}
