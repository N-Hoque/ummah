use adhan_rs::{from_csv, PrayerArguments, export_to_yaml, types::AdhanError};
use clap::StructOpt;

#[tokio::main]
async fn main() -> Result<(), AdhanError> {
    let args = PrayerArguments::parse();

    let month = from_csv(args).await?;

    let today = chrono::Local::now().date().naive_utc();

    let today = month.iter().find(|day| day.get_date() == today);

    if let Some(day) = today {
        println!("{}", day);
    }

    export_to_yaml(month)?;

    Ok(())
}
