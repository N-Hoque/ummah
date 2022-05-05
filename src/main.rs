use adhan::{
    arguments::PrayerArguments,
    core::{get_prayer_times, timetable_generator::TimetableGenerator, try_get_today},
    types::AdhanResult,
};
use clap::StructOpt;

#[tokio::main]
async fn main() -> AdhanResult<()> {
    let args = PrayerArguments::parse();

    let month = get_prayer_times(&args.settings()).await?;

    if args.is_today_only() {
        if let Some(day) = try_get_today(&month) {
            println!("{}", day);
        }
    } else if args.export_enabled() {
        let generator = TimetableGenerator::new(args.generate_default_css());
        generator.generate(&month)?;
    } else {
        for day in month {
            println!("{}", day);
        }
    }

    Ok(())
}
