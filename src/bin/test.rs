use std::{thread::sleep, time::Duration};

use steamdeck_input_rs::SteamdeckInput;

fn main() {
    let steamdeck_input = SteamdeckInput::new();

    loop {
        sleep(Duration::from_millis(1000));
        if let Some(state) = steamdeck_input.fetch() {
            println!("{state:?}");
        }
    }
}
