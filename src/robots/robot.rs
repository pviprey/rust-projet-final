use pathfinding::prelude::astar;
use crate::maps::map::{Map, Resource, Biome, TileInfo};
use crate::base::base::Base;
use ratatui::style::{Style, Color, Modifier};
use ratatui::text::Span;
use std::time::{Duration};

#[derive(Clone)]
pub struct Robot {
    pub x: i32,
    pub y: i32,
    map_width: i32,
    map_height: i32,
    pub energy: i32,
    pub iron_collected: i32,
    pub research_collected: i32,
    pub known_map: Map,
    pub path: Option<Vec<(i32, i32)>>,
    pub move_cooldown: f32,
    pub modules: Option<String>,
    pub modified: bool,
    pub class: Option<String>,
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
            path: None,
            move_cooldown: 0.0,
            modules: None,
            modified: false,
            class: None,
        }
    }

    pub fn collect_iron(&mut self, map: &mut Map) {           
            self.known_map.blueprint[self.x as usize][self.y as usize].resource = Resource::None;
            map.blueprint[self.x as usize][self.y as usize].resource = Resource::None;
    }

    pub fn collect_research(&mut self, map: &mut Map) {
            self.known_map.blueprint[self.x as usize][self.y as usize].resource = Resource::None;
            map.blueprint[self.x as usize][self.y as usize].resource = Resource::None;
    }

    pub fn moving(&mut self, deplacement: Option<(Vec<(i32, i32)>, u32)>) {
        if let Some((mut path, _)) = deplacement {
            if !path.is_empty() {
                if self.modules.as_deref() == Some("wheels") && path.len() >= 2 {
                    let (_bonus_x, _bonus_y) = path.remove(0);
                    let (next_x, next_y) = path.remove(0);
                    self.x = next_x;
                    self.y = next_y;
                    self.energy -= 1;
                } else {
                    let (next_x, next_y) = path.remove(0);
                    self.x = next_x;
                    self.y = next_y;
                    self.energy -= 1;
                }
                self.path = Some(path);
            }
        }
    }

    pub fn update(&mut self, map: &mut Map, base: &mut Base, delta_time: Duration) {
        self.move_cooldown -= delta_time.as_secs_f32();

        if self.move_cooldown > 0.0 {
            return;
        }

        self.move_cooldown = 1.0;

        if self.energy <= 0 {
            return;
        }

        if self.x == base.x && self.y == base.y {
            base.deposit_resources(self);
            base.recharge_robot(self);

            if !self.modified {
                base.modify_robot_equipment(self);
            }

            if self.energy < 95 {
                self.modified = false;
                return;
            }
        }

        if let Some((path_to_base, base_cost)) = self.path_finding(base.x, base.y, map) {
            if self.energy < (base_cost as i32 + 5) {
                if !path_to_base.is_empty() && path_to_base[0] == (self.x, self.y) {
                    let mut path = path_to_base.clone();
                    path.remove(0);
                    self.path = Some(path.clone());
                    self.moving(Some((path, base_cost)));
                } else {
                    self.path = Some(path_to_base.clone());
                    self.moving(Some((path_to_base, base_cost)));
                }
                return;
            }
        }

        let target_resource = match self.class.as_deref() {
            Some("scientist") => Resource::Research,
            _ => Resource::Iron,

        };

        if let Some(mut stored_path) = self.path.take() {
            if !stored_path.is_empty() {
                self.moving(Some((stored_path.clone(), 0)));
                return;
            }
        }

        let mut closest: Option<(Vec<(i32, i32)>, u32)> = None;
        for (ix, row) in map.blueprint.iter().enumerate() {
            for (iy, tile) in row.iter().enumerate() {
                if tile.resource == target_resource {
                    if let Some(p) = self.path_finding(ix as i32, iy as i32, map) {
                        if closest.is_none() || p.1 < closest.as_ref().unwrap().1 {
                            closest = Some(p);
                        }
                    }
                }
            }
        }

        if let Some((mut path, cost)) = closest {
            if !path.is_empty() && path[0] == (self.x, self.y) {
                path.remove(0);
            }
            self.path = Some(path.clone());
            self.moving(Some((path, cost)));
        }
    }

    pub fn discover_current_location(&mut self, biome: Biome, resource: Resource) {
        self.known_map.discover_area(self.x as usize, self.y as usize, 2, biome, resource);
    }

    pub fn move_random(&mut self, map: &Map) {
        if self.energy <= 0 {
            return;
        }

        if let Some(mut stored_path) = self.path.take() {
            if !stored_path.is_empty() {
                self.moving(Some((stored_path.clone(), 0)));
                return;
            }
        }

        let mut closest: Option<(Vec<(i32, i32)>, u32)> = None;
        for (ix, row) in map.blueprint.iter().enumerate() {
            for (iy, tile) in row.iter().enumerate() {
                if let Resource::Iron = tile.resource {
                    if let Some(p) = self.path_finding(ix as i32, iy as i32, map) {
                        if closest.is_none() || p.1 < closest.as_ref().unwrap().1 {
                            closest = Some(p);
                        }
                    }
                }
            }
        }

        if let Some((mut path, cost)) = closest {
            if !path.is_empty() && path[0] == (self.x, self.y) {
                path.remove(0);
            }
            self.path = Some(path.clone());
            self.moving(Some((path, cost)));
        }
    }
    
    pub fn path_finding(&self, dest_x: i32, dest_y: i32, map: &Map) -> Option<(Vec<(i32, i32)>, u32)> {
        let start = (self.x, self.y);
        let goal = (dest_x, dest_y);
        astar(
            &start,
            |&(x, y)| {
                let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];
                let mut neighbors = Vec::new();
                for &(dx, dy) in &deltas {
                    let next = (x + dx, y + dy);
                    if next.0 < 0 || next.1 < 0 || next.0 >= map.width || next.1 >= map.height {
                        continue;
                    }
                    let tile = &map.blueprint[next.0 as usize][next.1 as usize];
                    if let Biome::Mountain = tile.biome {
                        if self.modules.as_deref() != Some("tracks") {
                            continue;
                        }
                    }
                    if let Biome::Water = tile.biome {
                        if self.modules.as_deref() != Some("buoy") {
                            continue;
                        }
                    }
                    neighbors.push((next, 1));
                }
                neighbors
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
                .add_modifier(Modifier::BOLD)
        );
    }
}