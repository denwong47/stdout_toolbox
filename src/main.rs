mod modifiers;
#[allow(unused_imports)]
use modifiers::*;

mod progress;
use progress::*;

mod text;
use stdout_toolbox::ModifierIter;
use text::*;

use std::{thread, time};

fn main() {
    // =================================================================================
    // TEST WRAPPER

    // let style1 = {
    //     ForegroundColours::R0G1B2
    //         .join(BackgroundColours::R3G1B4)
    //         .join(Intensity::Bold)
    // }
    // .wrapper();

    // let s = String::from("Hello, World!");
    // println!("{}", style1(&s));

    // =================================================================================
    // TEST PROGRESS BAR

    let bar = ProgressStyle::GrayscaleToWhite.bar(60, Some(6));

    let size: usize = 2000;

    (0..size)
        .into_iter()
        .progressed(&bar, size, Some("Progressing... "), None)
        .for_each(|_| thread::sleep(time::Duration::from_micros(1000)));

    // println!("\nFinished.");

    // =================================================================================
    // TEST iter_member_in_str

    // let s = format!(
    //     "Hello I am {}Blue{} and {}{}BLUUUUUE{}{}.",
    //     ForegroundColours::Blue,
    //     ForegroundColours::Reset,
    //     BackgroundColours::Blue,
    //     ForegroundColours::BrightCyan,
    //     ForegroundColours::BrightCyan.resetter(),
    //     BackgroundColours::Blue.resetter(),
    // );

    // println!("{:?}", s);

    // let i = ForegroundColours::iter_member_in_str(&s);
    // i.for_each(
    //     | m | println!("Member found: {:?}", m)
    // )
}
