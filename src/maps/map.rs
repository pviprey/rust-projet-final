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
            if noise < -0.3 {
                print!("#");
            } else if noise < -0.1 {
                print!("F");
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
    
    let mut iron_collected = vec![vec![false; map_height]; map_width];
    
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        
        let noise_at_robot = perlin.get([robot.x as f64 / 10.0, robot.y as f64 / 10.0, 0.0]);
        if noise_at_robot < -0.1 && noise_at_robot >= -0.3 && !iron_collected[robot.x][robot.y] {
            robot.collect_iron();
            iron_collected[robot.x][robot.y] = true;
        }
        
        // status
        println!("Iron Collected: {}", robot.iron_collected);
        println!();
        
        // map
        for x in 0..map_width {
            for y in 0..map_height {
                if robot.x == x && robot.y == y {
                    print!("R");
                } else {
                    let noise = perlin.get([x as f64 / 10.0, y as f64 / 10.0, 0.0]);
                    if noise < -0.3 {
                        print!("#");
                    } else if noise < -0.1 {
                        if !iron_collected[x][y] {
                            print!("F");
                        } else {
                            print!(".");
                        }
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