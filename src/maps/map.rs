use noise::{NoiseFn, Perlin};
use std::thread;
use std::time::Duration;
use crate::robots::robot::Robot;

// base gen map
pub fn generate_map() {
    let seed = 0;
    let perlin = Perlin::new(seed);
    for x in 0..20 {
        for y in 0..100 {
            let noise = perlin.get([x as f64 / 10.0, y as f64 / 10.0, 0.0]);
            if noise < 0.0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn generate_map_with_robot() {
    let seed = 0;
    let perlin = Perlin::new(seed);
    let map_width = 20;
    let map_height = 100;
    let mut robot = Robot::new(map_width, map_height);
    
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        
        for x in 0..map_width {
            for y in 0..map_height {
                if robot.x == x && robot.y == y {
                    print!("R");
                } else {
                    let noise = perlin.get([x as f64 / 10.0, y as f64 / 10.0, 0.0]);
                    if noise < 0.0 {
                        print!("#");
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