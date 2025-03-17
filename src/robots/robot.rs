use rand::Rng;

pub struct Robot {
    pub x: usize,
    pub y: usize,
    map_width: usize,
    map_height: usize,
    energy: i32,
    pub iron_collected: i32,
}

impl Robot {
    pub fn new(map_width: usize, map_height: usize) -> Self {
        Self {
            x: map_width / 2,
            y: map_height / 2,
            map_width,
            map_height,
            energy: 100,
            iron_collected: 0,
        }
    }
    
    pub fn collect_iron(&mut self) {
        self.iron_collected += 1;
    }
    
    pub fn move_random(&mut self) {
        if self.energy <= 0 {
            return;
        } else {
            self.energy -= 1;
        }
        let mut rng = rand::rng();
        // 0: haut, 1: droite, 2: bas, 3: gauche
        match rng.random_range(0..4) {
        0 if self.y > 0 => self.y -= 1,
        1 if self.x < self.map_width - 1 => self.x += 1,
        2 if self.y < self.map_height - 1 => self.y += 1,
        3 if self.x > 0 => self.x -= 1,
            _ => {},
        }
    }
}