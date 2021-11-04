use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() {
    const REPITITIONS: u8 = 10;

    for i in 1..=REPITITIONS {
        println!(
            "Bend your wrist down over the edge ({} of {}).",
            i, REPITITIONS
        );
        count_down(5);
        println!("Release.");
        count_down(3);
    }

    for i in 1..=REPITITIONS {
        println!(
            "Raise your hand up off the table ({} of {}).",
            i, REPITITIONS
        );
        count_down(5);
        println!("Release.");
        count_down(3);
    }

    for i in 1..=REPITITIONS {
        println!(
            "Move your hand toward your thumb's side ({} of {}).",
            i, REPITITIONS
        );
        count_down(5);
        println!("Release.");
        count_down(3);
        println!(
            "Move your hand toward your litle finger's side ({} of {}).",
            i, REPITITIONS
        );
        count_down(5);
        println!("Release.");
        count_down(3);
    }

    for i in 1..=REPITITIONS {
        println!("Turn your palm up ({} of {}).", i, REPITITIONS);
        count_down(5);
        println!("Release.");
        count_down(3);
        println!("Turn your palm down ({} of {}).", i, REPITITIONS);
        count_down(5);
        println!("Release.");
        count_down(3);
    }

    println!("Rotate hand {} times.", REPITITIONS);
}

fn count_down(n: u8) {
    for i in (1..=n).rev() {
        print!("\r{}", i);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    print!("\r");
}
