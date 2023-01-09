mod colours;
use colours::ANSI256Colours as C;
use colours::ANSIIntensity as B;

fn main() {
    let style1 = colours::colouriser(Some(C::BrightWhite), Some(C::R0G3B4), Some(B::Bold));
    let style2 = colours::colouriser(Some(C::R4G5B5), None, Some(B::Bold));
    
    let s = String::from("Hello, World!");
    println!("{}", style1(&s));
    println!("{}", style2(&s));
}
