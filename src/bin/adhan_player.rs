use clap::StructOpt;
use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::File,
    io::{stdout, BufReader, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

fn decode_audio<P: AsRef<std::path::Path>>(path: P) -> Decoder<BufReader<File>> {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path).expect("Buffering file"));
    // Decode that sound file into a source
    Decoder::new(file).expect("Decoding file")
}

fn play_audio(source: Decoder<BufReader<File>>, running: Arc<AtomicBool>) {
    let (_stream, stream_handle) =
        OutputStream::try_default().expect("Loading default audio stream");
    let sink = Sink::try_new(&stream_handle).expect("Creating audio sink");
    sink.append(source);
    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
    running.store(false, Ordering::Relaxed);
}

fn display_progress(running: Arc<AtomicBool>) {
    let mut counter = 0;
    while running.load(Ordering::Relaxed) {
        match counter % 3 {
            0 => print!("Calling the Adhan.  \r"),
            1 => print!("Calling the Adhan.. \r"),
            _ => print!("Calling the Adhan...\r"),
        }
        stdout().flush().expect("Flushing IO buffer");
        print!("{:48}\r", "");
        thread::sleep(Duration::from_secs(1));
        counter += 1;
    }
}

#[tokio::main]
async fn main() {
    let args = adhan::arguments::PrayerArguments::parse();

    let month = adhan::core::get_prayer_times(&args.settings())
        .await
        .expect("Getting month");

    let times_today = month.today().expect("Getting today");

    let adhan_audio_path = adhan::core::fs::get_user_filepath().join("adhan.mp3");

    for prayer in times_today.get_prayers() {
        let prayer_name = prayer.get_name();
        let prayer_time = prayer.get_time();

        loop {
            let now = chrono::Local::now().time();
            if now >= prayer_time {
                let decoded_audio = decode_audio(&adhan_audio_path);

                let running = Arc::new(AtomicBool::new(true));
                let r1 = running.clone();
                let audio_thread = thread::spawn(move || play_audio(decoded_audio, r1));

                display_progress(running);

                audio_thread.join().expect("Joining Audio Thread");

                break;
            }
            print!("{} starts at {}\r", prayer_name, prayer_time);
            stdout().flush().expect("Flushing IO buffer");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
