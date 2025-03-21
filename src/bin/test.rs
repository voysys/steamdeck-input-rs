use steamdeck_input_rs::SteamdeckInput;

fn main() {
    let steamdeck_input = SteamdeckInput::new();

    loop {
        if let Some(state) = steamdeck_input.state() {
            println!("{state:?}");
        }
    }
}
