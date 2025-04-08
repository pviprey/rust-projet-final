use crate::robots::robot::Robot;
use crate::maps::map::{Map, Resource, Biome};

pub struct Base {
    pub energy_capacity: i32,
    pub energy: i32,
    pub iron_storage: i32,
    pub iron: i32,
    pub research_storage: i32,
    pub research: i32,
    pub x : i32,
    pub y : i32,
    pub lvl : i32,
}

impl Base {
    pub fn new(map_width: i32, map_height: i32) -> Self {
        Self {
            energy_capacity: 500,
            energy: 250,
            iron_storage: 100,
            iron: 0,
            research_storage: 0,
            research: 0,
            x: map_width / 2,
            y: map_height / 2,
            lvl: 1,
        }
    }

    pub fn recharge_robot(&mut self, robot: &mut Robot) {
        if self.energy > 0 {
            robot.energy += 5;
            self.energy -= 5;
        }
    }

    pub fn generate_energy(&mut self) {
        if self.energy < self.energy_capacity {
            self.energy += 1;
        }
    }

    pub fn deposit_resources(&mut self, robot: &mut Robot) {
        if robot.iron_collected > 0 {
            self.iron = robot.iron_collected;
            robot.iron_collected = 0;
        }
        if robot.research_collected > 0 {
            self.research = robot.research_collected;
            robot.research_collected = 0;
        }
        self.energy -= 10;
    }

    pub fn share_map(&self, robot: &mut Robot) {
    }

    pub fn modify_robot_equipment(&mut self, robot: &mut Robot) {
        let radius = 10;
        let blueprint = &robot.known_map.blueprint;
        let map_width = blueprint.len();
        if map_width == 0 {
            return;
        }
        let map_height = blueprint[0].len();

        let base_x = self.x as usize;
        let base_y = self.y as usize;
        let start_x = if base_x >= radius { base_x - radius } else { 0 };
        let start_y = if base_y >= radius { base_y - radius } else { 0 };
        let end_x = usize::min(base_x + radius, map_width - 1);
        let end_y = usize::min(base_y + radius, map_height - 1);

        let mut water_count = 0;
        let mut mountain_count = 0;
        let mut others_count = 0;

        for i in start_x..=end_x {
            for j in start_y..=end_y {
                match blueprint[i][j].biome {
                    Biome::Water => water_count += 1,
                    Biome::Mountain => mountain_count += 1,
                    _ => others_count += 1,
                }
            }
        }

        if water_count >= mountain_count && water_count >= others_count {
            robot.modules = Some("buoy".to_string());
        } else if mountain_count >= water_count && mountain_count >= others_count {
            robot.modules = Some("tracks".to_string());
        } else {
            robot.modules = Some("wheels".to_string());
        }
    }

    pub fn upgrade_base(&mut self) {
    }
} 