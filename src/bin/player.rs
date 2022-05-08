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

fn main() {
    let docs = adhan::core::fs::get_user_filepath().join("adhan.mp3");

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(
        File::open(docs).expect("Buffering file"),
    );
    // Decode that sound file into a source
    let source = Decoder::new(file).expect("Decoding file");

    let running = Arc::new(AtomicBool::new(true));
    let r1 = running.clone();

    let th = thread::spawn(move || {
        let (_stream, stream_handle) =
            OutputStream::try_default().expect("Loading default audio stream");

        let sink = Sink::try_new(&stream_handle).expect("Creating audio sink");
        sink.append(source);

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
        r1.store(false, Ordering::Relaxed);
    });

    let mut counter = 0;

    while running.load(Ordering::Relaxed) {
        match counter % 3 {
            0 => print!("Playing Audio.  \r"),
            1 => print!("Playing Audio.. \r"),
            _ => print!("Playing Audio...\r"),
        }
        stdout().flush().expect("Flushing IO buffer");
        print!("{:32}\r", "");
        thread::sleep(Duration::from_secs(1));
        counter += 1;
    }

    th.join().expect("Joining Audio Thread");
}
