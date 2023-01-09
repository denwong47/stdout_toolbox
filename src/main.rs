mod colours;
use colours::*;

fn main() {
    let style1 = {
        colours::ForegroundColours::R0G1B2
        .join(colours::BackgroundColours::R3G1B4)
        .join(colours::ANSIIntensity::Bold)
    }.wrapper();

    let s = String::from("Hello, World!");
    println!("{}", style1(&s));
}
