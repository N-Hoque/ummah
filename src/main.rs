use adhan_rs::from_csv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let month = from_csv().await?;

    let today = chrono::Utc::now();

    for day in month {
        if day.get_date() == today.date().naive_local() {
            println!("{}", day);
        }
    }

    Ok(())
}
