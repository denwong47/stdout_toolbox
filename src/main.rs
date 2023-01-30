mod modifiers;
#[allow(unused_imports)]
use modifiers::*;

mod progress;
use progress::*;

mod text;
// use text::*;

use std::{thread, time};

fn main() {
    // let style1 = {
    //     ForegroundColours::R0G1B2
    //         .join(BackgroundColours::R3G1B4)
    //         .join(Intensity::Bold)
    // }
    // .wrapper();

    // let s = String::from("Hello, World!");
    // println!("{}", style1(&s));

    let bar = ProgressStyle::GrayscaleToWhite.bar(60, Some(6));

    let size: usize = 2000;

    (0..size)
        .into_iter()
        .progressed(&bar, size, Some("Progressing... "), None)
        .for_each(|_| thread::sleep(time::Duration::from_micros(1000)));

    println!("\nFinished.");
}
