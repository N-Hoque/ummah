use adhan_rs::{get_prayer_times, try_get_today, types::AdhanResult, PrayerArguments};
use clap::StructOpt;

#[tokio::main]
async fn main() -> AdhanResult<()> {
    let args = PrayerArguments::parse();

    let month = get_prayer_times(args).await?;

    let today = try_get_today(&month);

    if let Some(day) = today {
        println!("{}", day);
    }

    Ok(())
}
