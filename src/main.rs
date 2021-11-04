use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() {
    for _ in 0..10 {
        println!("Bend your wrist down over the edge.");
        count_down(5);
        println!("Release.");
        count_down(3);
    }

    for _ in 0..10 {
        println!("Raise your hand up off the table.");
        count_down(5);
        println!("Release.");
        count_down(3);
    }

    for _ in 0..10 {
        println!("Move your hand toward your thumb's side.");
        count_down(5);
        println!("Release.");
        count_down(3);
        println!("Move your hand toward your litle finger's side.");
        count_down(5);
        println!("Release.");
        count_down(3);
    }

    for _ in 0..10 {
        println!("Turn your palm up.");
        count_down(5);
        println!("Release.");
        count_down(3);
        println!("Turn your palm down.");
        count_down(5);
        println!("Release.");
        count_down(3);
    }

    println!("Rotate hand ten times.");
}

fn count_down(n: u8) {
    for i in (1..=n).rev() {
        print!("\r{}", i);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    print!("\r");
}
