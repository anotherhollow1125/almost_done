use std::cmp::min;
use std::thread;
use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

fn pure_pb(
    m: &mut MultiProgress,
    template: &str,
    msg: &'static str,
    total_size: u64,
    speed: u64,
    prog_char: Option<&str>,
) -> thread::JoinHandle<()> {
    let pb = m.add(ProgressBar::new(total_size));
    pb.set_style(
        ProgressStyle::with_template(template)
            .unwrap()
            .progress_chars(prog_char.unwrap_or("#>-")),
    );
    pb.set_message(msg);

    thread::spawn(move || {
        let mut downloaded = 0;

        while downloaded < total_size {
            let new = min(downloaded + speed, total_size);
            downloaded = new;
            pb.set_position(new);
            thread::sleep(Duration::from_millis(12));
        }

        pb.finish_with_message("downloaded");
    })
}

fn stop_pb(
    m: &mut MultiProgress,
    template: &str,
    msg: &'static str,
    total_size: u64,
    speed: u64,
    slow_speed: u64,
    prog_char: Option<&str>,
) -> thread::JoinHandle<()> {
    let pb = m.add(ProgressBar::new(total_size));
    pb.set_style(
        ProgressStyle::with_template(template)
            .unwrap()
            .progress_chars(prog_char.unwrap_or("#>-")),
    );
    pb.set_message(msg);

    thread::spawn(move || {
        let mut downloaded = 0;
        let mut s = speed;
        let mut wait;

        while downloaded < total_size {
            let new = min(downloaded + s, total_size);
            downloaded = new;
            pb.set_position(new);
            if total_size - downloaded >= speed * 2 {
                s = speed;
                wait = 12;
            } else {
                s = slow_speed;
                wait = 1200;
            };

            thread::sleep(Duration::from_millis(wait));
        }

        pb.finish_with_message("downloaded");
    })
}

fn main() {
    let mut m = MultiProgress::new();
    let template = "{spinner:.green} [{elapsed_precise}] {msg} {percent_precise:>7}% [{bar:20.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";
    let h1 = pure_pb(&mut m, template, "contents 1", 1000, 1, None);
    thread::sleep(Duration::from_millis(500));
    let h2 = pure_pb(&mut m, template, "contents 2", 1000, 10, Some("|:_"));
    thread::sleep(Duration::from_millis(500));
    let h3 = stop_pb(&mut m, template, "contents 3", 1000000, 5000, 1, None);

    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
}
