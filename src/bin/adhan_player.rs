use adhan::{
    core::{clear_cache, get_prayer_times, update_timetable},
    types::{AdhanError, AdhanResult, PrayerName},
};
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

fn decode_audio<P: AsRef<std::path::Path>>(path: P) -> AdhanResult<Decoder<BufReader<File>>> {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(path)?);
    // Decode that sound file into a source
    Decoder::new(file).map_err(AdhanError::Decode)
}

fn play_audio(source: Decoder<BufReader<File>>, running: Arc<AtomicBool>) -> AdhanResult<()> {
    let (_stream, stream_handle) = OutputStream::try_default().map_err(AdhanError::Stream)?;
    let sink = Sink::try_new(&stream_handle).map_err(AdhanError::Play)?;
    sink.append(source);
    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
    running.store(false, Ordering::Relaxed);
    Ok(())
}

fn display_progress(prayer_name: PrayerName, running: Arc<AtomicBool>) -> AdhanResult<()> {
    let mut counter = 0;
    print!("{:64}\r", "");
    while running.load(Ordering::Relaxed) {
        match counter % 3 {
            0 => print!("{} is starting.  \r", prayer_name),
            1 => print!("{} is starting.. \r", prayer_name),
            _ => print!("{} is starting...\r", prayer_name),
        }
        stdout().flush().map_err(AdhanError::IO)?;
        print!("{:64}\r", "");
        thread::sleep(Duration::from_secs(1));
        counter += 1;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> AdhanResult<()> {
    let args = adhan::argparser::arguments::PrayerArguments::parse();

    if args.clear_cache() {
        clear_cache()?;
    }

    let settings = args.settings();

    let mut month = get_prayer_times(&settings, args.month()).await?;

    let times_today = month.today_mut().expect("Getting today's times");

    let adhan_audio_path = adhan::core::fs::get_user_filepath().join("adhan.mp3");

    while let Some(prayer) = times_today.get_next_prayer_mut() {
        let prayer_name = prayer.get_name();
        let prayer_time = prayer.get_time();

        loop {
            let now = chrono::Local::now().time();
            print!(
                "Time now is {}. [{}] starts at {}\r",
                now.format("%H:%M:%S"),
                prayer_name,
                prayer_time
            );
            stdout().flush()?;
            if now >= prayer_time {
                let decoded_audio = decode_audio(&adhan_audio_path)?;

                let running = Arc::new(AtomicBool::new(true));
                let r1 = running.clone();
                let audio_thread =
                    thread::spawn(move || play_audio(decoded_audio, r1).expect("Playing audio"));

                display_progress(prayer_name, running)?;

                audio_thread.join().expect("Failed to join thread");

                prayer.set_performed();

                update_timetable(times_today)?;

                break;
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    Ok(())
}
