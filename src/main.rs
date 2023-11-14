use std::io::{stdin, stdout};
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
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
    #[arg(short, long, default_value = "25:00")]
    work: String,

    /// Duration for a short break in mm:ss
    #[arg(short, long, default_value = "5:00")]
    short: String,

    /// Duration for a long break in mm:ss
    #[arg(short, long, default_value = "15:00")]
    long: String,

    #[arg(short, long, default_value = "wswswl")]
    pattern: String,
}

fn main() {
    let args = PomArgs::parse();

    let work_duration = str_to_duration(&args.work);
    let short_duration = str_to_duration(&args.short);
    let long_duration = str_to_duration(&args.long);

    let work_bar = create_progress_bar(work_duration);
    let short_bar = create_progress_bar(short_duration);
    let long_bar = create_progress_bar(long_duration);

    let mut index = 0;
    let modulus = args.pattern.len();



    loop {
        clear_terminal();
        let step_idx = index % modulus;
        let step = &args
            .pattern
            .chars()
            .nth(step_idx)
            .expect("Failure to get pattern char!")
            .to_string();
        println!(
            "Pattern: {}; Index: {}; Step: {}",
            &args.pattern, step_idx, step
        );

        index += 1;

        let finished_callback:fn(ProgressBar) = |pb| {
            pb.finish_with_message("Step done!\nPress 'q' to continue");
            pb.reset();
        };

        match step.as_str() {
            "w" => {
                run_timer(work_bar.clone(), work_duration, finished_callback);
            }
            "s" => {
                run_timer(short_bar.clone(), short_duration, finished_callback);
            }
            "l" => {
                run_timer(long_bar.clone(), long_duration, finished_callback);
            }
            _ => {
                println!("Invalid pattern encountered!");
                break;
            }
        }

        play_sound_until_keypress();
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
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {percent}% {msg}")
            .expect("Failed to create progress bar!")
            .progress_chars("##-"),
    );
    bar
}

fn run_timer<F>(bar: ProgressBar, duration: Duration, on_finish: F)
where F: Fn(ProgressBar) {
    bar.reset();
    let sec = duration.as_secs() % 60;
    let min = (duration.as_secs() / 60) % 60;
    let bar_message = format!("Running for {:0>2}:{:0>2}\nctrl+c to quit", min.to_string(), sec.to_string());
    bar.set_message(bar_message);
    for _ in 0..duration.as_secs() {
        sleep(Duration::new(1, 0));
        bar.inc(1);
    }
    on_finish(bar);


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
                    break;
                }
                _ => continue,
            }
        }
    });

    while running.load(Ordering::SeqCst) {
        sleep(Duration::from_secs(1));
        Command::new("afplay")
            .arg("/System/Library/Sounds/Ping.aiff")
            .output()
            .expect("Failed to play sound");
    }
}
