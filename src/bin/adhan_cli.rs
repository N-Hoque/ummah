use adhan::{
    argparser::arguments::PrayerArguments,
    core::{clear_cache, get_prayer_times, timetable_generator::TimetableGenerator},
    types::AdhanResult,
};
use clap::StructOpt;

#[tokio::main]
async fn main() -> AdhanResult<()> {
    let args = PrayerArguments::parse();

    if args.clear_cache() {
        clear_cache()?;
    }

    let settings = args.settings();

    let month = get_prayer_times(&settings, args.month()).await?;

    if args.is_today_only() {
        if let Some(day) = month.today() {
            println!("{}", day);
        }
    } else if args.export_enabled() {
        let generator = TimetableGenerator::new(args.generate_default_css(), args.month());
        generator.generate(&month)?;
    } else {
        for day in month.iter() {
            println!("{}", day);
        }
    }

    Ok(())
}
