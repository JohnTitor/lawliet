use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::process;

static LAWLIET_DIR: &str = "LAWLIET_DIR";

fn main() {
    let sound_dir =
        env::var_os(LAWLIET_DIR).unwrap_or_else(|| panic!("`{}` must be set", LAWLIET_DIR));
    let sound_dir = sound_dir
        .to_str()
        .unwrap_or_else(|| panic!("failed to convert `{}` to `str`", LAWLIET_DIR));

    let mut cmd: Vec<String> = env::args().collect();
    let output = if cmd.len() >= 3 {
        let args: Vec<String> = cmd.drain(2..).collect();
        // This unwrap should be safe.
        let cmd = cmd.pop().unwrap();

        process::Command::new(cmd)
            .args(args)
            .output()
            .expect("failed to execute given command")
    } else if cmd.len() == 2 {
        // This unwrap should be safe.
        let cmd = cmd.pop().unwrap();

        process::Command::new(cmd)
            .output()
            .expect("failed to execute given command")
    } else {
        // If lawliet runs w/o args, print msg and exit with 1.
        eprintln!("lawliet should be run with other command");
        process::exit(1);
    };

    // Print std{out,err}.
    io::stdout()
        .write_all(&output.stdout)
        .expect("failed to write stdout");
    io::stderr()
        .write_all(&output.stderr)
        .expect("failed to write stderr");

    // Play sound following the status.
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("failed to get default output device");
    let sink = Sink::try_new(&stream_handle).expect("failed to build a new `Sink`");
    let file = File::open(if output.status.success() {
        format!("{}/success.mp3", sound_dir)
    } else {
        format!("{}/failure.mp3", sound_dir)
    })
    .expect("Failed to open audio file. See README for more info.");
    sink.append(Decoder::new(BufReader::new(file)).expect("failed to decode file"));
    sink.sleep_until_end();
}
