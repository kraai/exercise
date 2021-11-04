// Copyright 2021 Matthew James Kraai
//
// This file is part of exercise.
//
// exercise is free software: you can redistribute it and/or modify it
// under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// exercise is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with exercise.  If not, see
// <https://www.gnu.org/licenses/>.

use directories::ProjectDirs;
use std::{
    fs::{DirBuilder, OpenOptions},
    io::{self, Write},
    os::unix::fs::DirBuilderExt,
    thread,
    time::Duration,
};
use time::OffsetDateTime;

const REPITITIONS: u8 = 10;

fn main() {
    exercise("Bend your wrist down over the edge");
    exercise("Raise your hand up off the table");
    exercise2(
        "Move your hand toward your thumb's side",
        "Move your hand toward your little finger's side",
    );
    exercise2("Turn your palm down", "Turn your palm up");
    println!("Rotate hand {} times clockwise.", REPITITIONS);
    println!("Rotate hand {} times counterclockwise.", REPITITIONS);
    print!("Press Enter when finished.");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

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

fn exercise(description: &str) {
    let spaces = " ".repeat(description.len() + 19);

    for i in 1..=REPITITIONS {
        print!("\r{} ({} of {}): Ready.", description, i, REPITITIONS);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("\r{}", spaces);
        print!("\r{} ({} of {}): Set.", description, i, REPITITIONS);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        for j in (1..=5).rev() {
            print!("\r{}", spaces);
            print!("\r{} ({} of {}): {}", description, i, REPITITIONS, j);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        print!("\r{} ({} of {}): Reset.", description, i, REPITITIONS);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!();
}

fn exercise2(description1: &str, description2: &str) {
    let spaces1 = " ".repeat(description1.len() + 19);
    let spaces2 = " ".repeat(description2.len() + 19);

    for i in 1..=REPITITIONS {
        print!("\r{}", spaces2);
        print!("\r{} ({} of {}): Ready.", description1, i, REPITITIONS);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("\r{}", spaces1);
        print!("\r{} ({} of {}): Set.", description1, i, REPITITIONS);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        for j in (1..=5).rev() {
            print!("\r{}", spaces1);
            print!("\r{} ({} of {}): {}", description1, i, REPITITIONS, j);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        print!("\r{} ({} of {}): Reset.", description1, i, REPITITIONS);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("\r{}", spaces1);
        print!("\r{} ({} of {}): Ready.", description2, i, REPITITIONS);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        print!("\r{}", spaces2);
        print!("\r{} ({} of {}): Set.", description2, i, REPITITIONS);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
        for j in (1..=5).rev() {
            print!("\r{}", spaces2);
            print!("\r{} ({} of {}): {}", description2, i, REPITITIONS, j);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        print!("\r{} ({} of {}): Reset.", description2, i, REPITITIONS);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!();
}
