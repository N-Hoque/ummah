use adhan_rs::{get_prayer_times, try_get_today, types::AdhanResult, PrayerArguments};
use clap::StructOpt;

#[tokio::main]
async fn main() -> AdhanResult<()> {
    let args = PrayerArguments::parse();

    let month = get_prayer_times(&args.settings()).await?;

    if args.is_today_only() {
        if let Some(day) = try_get_today(&month) {
            println!("{}", day);
        }
    } else {
        for day in month {
            println!("{}", day);
        }
    }

    Ok(())
}
