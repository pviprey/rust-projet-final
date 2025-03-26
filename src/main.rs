mod maps;
mod robots;

fn main() {
    maps::map::generate_map_with_robot();

    let map = maps::map::generate_map();
}