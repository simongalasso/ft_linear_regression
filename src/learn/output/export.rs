use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::game::game::*;
use crate::maths::scale::*;

pub fn export_to_file(game: &Game) {
    let m = game.m.to_string();
    let b = scale(game.b as f64, 0.0, 1.0, 0.0, game.max_values.y).to_string();
    let output = format!("m,b\n{:.10},{:.10}", m, b);
    let path = Path::new("output/save.csv");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(output.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
