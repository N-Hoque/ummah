use adhan_rs::{get_prayer_times, types::AdhanResult, PrayerArguments};
use clap::StructOpt;

#[tokio::main]
async fn main() -> AdhanResult<()> {
    let args = PrayerArguments::parse();

    let month = get_prayer_times(args).await?;

    let today = chrono::Local::now().date().naive_utc();

    let today = month.iter().find(|day| day.get_date() == today);

    if let Some(day) = today {
        println!("{}", day);
    }

    Ok(())
}
