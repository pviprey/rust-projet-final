use pathfinding::prelude::astar;
use rand::Rng;

pub struct Robot {
    pub x: i32,
    pub y: i32,
    map_width: i32,
    map_height: i32,
    pub energy: i32,
    pub iron_collected: i32,
    pub research_collected: i32,
}

impl Robot {
    pub fn new(map_width: i32, map_height: i32) -> Self {
        Self {
            x: map_width / 2,
            y: map_height / 2,
            map_width,
            map_height,
            energy: 100,
            iron_collected: 0,
            research_collected: 0,
        }
    }
    
    pub fn collect_iron(&mut self) {
        self.iron_collected += 1;
    }

    pub fn collect_research(&mut self) {
        self.research_collected += 1;
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

    pub fn moving(&mut self, deplacement: Option<(Vec<(i32, i32)>, u32)>){
        
    }

    pub fn path_finding(&mut self, dest_x: i32, dest_y: i32) -> Option<(Vec<(i32, i32)>, u32)> {
        let start = (self.x, self.y);
        let goal = (dest_x, dest_y);
        
        astar(&start, |&(x, y)| {
            let deltas: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

            deltas.iter().map(|&(dx, dy)| ((x + dx, y + dy), 1)).collect::<Vec<_>>()
            },
            |&(x,y )| goal.0.abs_diff(x) + goal.1.abs_diff(y),
            |&p| p == goal,
        )
    }
}