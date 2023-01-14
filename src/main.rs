mod modifiers;
use modifiers::*;

fn main() {
    let style1 = {
        ForegroundColours::R0G1B2
            .join(BackgroundColours::R3G1B4)
            .join(Intensity::Bold)
    }
    .wrapper();

    let s = String::from("Hello, World!");
    println!("{}", style1(&s));
}
