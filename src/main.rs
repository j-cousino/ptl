//! # ptl - project time logger
//!
//! A small command line utility I use to keep track of how much time
//! I've spent on projects. It keeps a file full of time stamps
//!
//! ## Use
//!
//! Start working on a project, while in your main project directory
//! ```console, no_run
//! ptl start
//! ```
//!
//! time is logged in a file named ptl.toml simple toml file
//!
//! Stop working on the project
//! ```console, no_run
//! ptl stop
//! ```
//!
//! Report the total time spent
//! ```console, no_run
//! ptl total
//! ```
//!
mod config;
mod timelog;

use std::fs;
use std::path::Path;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Cli {
    Start,
    Stop,
    Total,
}

/// Entry point
///
/// Use StructOpt to parse the commandline and then calls the
/// sub-command handler.
fn main() {
    let args = Cli::from_args();

    match args {
        Cli::Start => do_start(),
        Cli::Stop => do_stop(),
        Cli::Total => do_total(),
    }
}

fn do_start() {
    let path = Path::new("plt.toml");
    let config = config::Config::gather();
    let mut timelog = timelog::Timelog::new(config.email());

    // See if the file exists if it does load it.
    if path.exists() {
        let toml_str = fs::read_to_string(path).unwrap();
        timelog = toml::from_str(&toml_str).unwrap();

        // Of the last entry doesnt have a stop time give a warning
        if timelog.entries[timelog.entries.len() - 1].stop == None {
            println!("Warning: Last entry hasn't been completed!");
            return;
        }
        // Otherwise create a new entry
        timelog.entries.push(timelog::LogEntry::new(config.email()));
    }

    let toml_str = toml::to_string(&timelog);
    fs::write(path, toml_str.unwrap()).unwrap();
}

fn do_stop() {
    let path = Path::new("plt.toml");
    let config = config::Config::gather();
    let mut timelog = timelog::Timelog::new(config.email());

    // See if the file exists if it does load it.
    if path.exists() {
        let toml_str = fs::read_to_string(path).unwrap();
        timelog = toml::from_str(&toml_str).unwrap();

        // If the last entry has a stop time give a warning
        let idx = timelog.entries.len() - 1;

        if timelog.entries[idx].stop != None ||
            timelog.entries[idx].email != *config.email() {
            println!("Warning: Haven't started working on this project!");
            return;
        }
        // Otherwise create stop entry
        timelog.entries[idx].stop = Some(chrono::Local::now());
    }else {
        return;
    }

    let toml_str = toml::to_string(&timelog);
    fs::write(path, toml_str.unwrap()).unwrap();
}

fn do_total() {
    let path = Path::new("plt.toml");
    let config = config::Config::gather();
    let email = config.email();

    // See if the file exists if it does load it.
    if path.exists() {
        let toml_str = fs::read_to_string(path).unwrap();
        let timelog: timelog::Timelog = toml::from_str(&toml_str).unwrap();

        let mut duration = chrono::Duration::days(0);
        for elem in timelog.entries.iter() {
            if elem.email == *email {
                duration = duration + (elem.stop.unwrap_or(elem.start) - elem.start);
            }
        }
        println!(
            "{} {}:{} H:M",
            email,
            duration.num_hours(),
            duration.num_minutes()
        );
    } else {
        println!("Warning: No time log exists")
    }
}
