use crate::robots::robot::Robot;

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
            iron_storage: 0,
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
            robot.energy += 1;
            self.energy -= 1;
        }
    }

    pub fn generate_energy(&mut self) {
        if self.energy < self.energy_capacity {
            self.energy += 1;
        }
    }

    pub fn deposit_resources(&mut self, robot: &mut Robot) {
    }

    pub fn share_map(&self, robot: &mut Robot) {
    }

    pub fn modify_robot_equipment(&mut self, robot: &mut Robot) {
    }

    pub fn upgrade_base(&mut self) {
    }
} 