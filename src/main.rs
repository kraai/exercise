use directories::ProjectDirs;
use std::{
    fs::{DirBuilder, OpenOptions},
    io::{self, Write},
    os::unix::fs::DirBuilderExt,
    thread,
    time::Duration,
};
use time::OffsetDateTime;

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

    println!(
        "Rotate hand {} times clockwise and {} time counterclockwise.",
        REPITITIONS, REPITITIONS
    );

    let proj_dirs = ProjectDirs::from("org.ftbfs", "", "exercise").unwrap();
    let data_dir = proj_dirs.data_dir();
    DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(data_dir)
        .unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(data_dir.join("log"))
        .unwrap();
    writeln!(file, "{}", OffsetDateTime::now_local().unwrap()).unwrap();
}

fn count_down(n: u8) {
    for i in (1..=n).rev() {
        print!("\r{}", i);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    print!("\r");
}
