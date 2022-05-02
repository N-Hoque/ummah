use adhan_rs::{from_csv, PrayerArguments};
use clap::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = PrayerArguments::parse();

    let month = from_csv(args).await?;

    let mut this_month = std::fs::File::create("this_month.yaml").expect("Creating new file");

    serde_yaml::to_writer(&mut this_month, &month).expect("Writing to file");

    let today = chrono::Local::now().date().naive_utc();

    let today = month.into_iter().find(|day| day.get_date() == today);

    if let Some(day) = today {
        println!("{}", day);
    }

    Ok(())
}
