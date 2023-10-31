// Import IntoRawMode
use std::io::{stdin, stdout};
use std::process::Command;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Your Name", about = "Pomodoro Timer")]
struct PomArgs {
    /// Duration for work in mm:ss
    #[arg(short, long)]
    work: String,

    /// Duration for a short break in mm:ss
    #[arg(short, long)]
    short: String,

    /// Duration for a long break in mm:ss
    #[arg(short, long)]
    long: String,
}

fn main() {
    let args = PomArgs::parse();

    let work_duration = str_to_duration(&args.work);
    let short_duration = str_to_duration(&args.short);
    let long_duration = str_to_duration(&args.long);

    let mut work_bar = create_progress_bar(work_duration);
    let mut short_bar = create_progress_bar(short_duration);
    let mut long_bar = create_progress_bar(long_duration);

    loop {
        // Work phase
        clear_terminal();
        println!("Work Time!");
        run_timer(work_bar.clone(), work_duration);
        play_sound_until_keypress();

        // Short break phase
        clear_terminal();
        println!("Short Break!");
        run_timer(short_bar.clone(), short_duration);
        play_sound_until_keypress();

        // Work phase
        clear_terminal();
        println!("Work Time!");
        run_timer(work_bar.clone(), work_duration);
        play_sound_until_keypress();


        // Short break phase
        clear_terminal();
        println!("Short Break!");
        run_timer(short_bar.clone(), short_duration);
        play_sound_until_keypress();

        // Work phase
        clear_terminal();
        println!("Work Time!");
        run_timer(work_bar.clone(), work_duration);
        play_sound_until_keypress();

        // Long break phase
        clear_terminal();
        println!("Long Break!");
        run_timer(long_bar.clone(), long_duration);
        play_sound_until_keypress();

        // Reset progress bars at the end of a cycle
        work_bar = create_progress_bar(work_duration);
        short_bar = create_progress_bar(short_duration);
        long_bar = create_progress_bar(long_duration);
    }

}

fn str_to_duration(time: &str) -> Duration {
    let parts: Vec<&str> = time.split(':').collect();
    let minutes: u64 = parts[0].parse().expect("Invalid minutes");
    let seconds: u64 = parts[1].parse().expect("Invalid seconds");
    Duration::new(minutes * 60 + seconds, 0)
}

fn create_progress_bar(duration: Duration) -> ProgressBar {
    let bar = ProgressBar::new(duration.as_secs());
    bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent}% {msg}").expect("Failed to create progress bar!")
        .progress_chars("##-"));
    bar
}

fn run_timer(mut bar: ProgressBar, duration: Duration) {
    bar.set_message("Running...");
    for _ in 0..duration.as_secs() {
        sleep(Duration::new(1, 0));
        bar.inc(1);
    }
    bar.finish_with_message("Done!");
    bar.reset();
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}


fn play_sound_until_keypress() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    thread::spawn(move || {
        let stdin = stdin();
        let _stdout = stdout().into_raw_mode().unwrap(); // Enter raw mode

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    r.store(false, Ordering::SeqCst);
                    break
                },
                _ => continue,
            }
        }
    });

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(1));
        Command::new("afplay")
            .arg("/System/Library/Sounds/Ping.aiff")
            .output()
            .expect("Failed to play sound");

    }
}

