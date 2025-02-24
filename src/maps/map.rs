use noise::{NoiseFn, Perlin};

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