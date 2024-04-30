mod database;
mod settings;

use settings::constants::Constants;

fn main() {
    let constants = Constants::new();
    println!("{:#?}", constants);
}
