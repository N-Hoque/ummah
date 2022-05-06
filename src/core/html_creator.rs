use crate::{
    day::Day,
    types::{AdhanError, AdhanResult},
};

use chrono::NaiveDate;
use html_builder::Html5;

use std::fmt::Write;

use super::fs::{get_user_filepath, write_file};

pub fn create_table(html: &mut html_builder::Node, month: &[Day]) -> AdhanResult<()> {
    let mut body = html.body();

    writeln!(body.h1(), "Adhan").map_err(|x| AdhanError::Unknown(Box::new(x)))?;

    let mut table = body.table().attr("class='tg'");
    create_table_header(&mut table, month[0].get_date())?;
    create_table_body(&mut table, month)?;
    Ok(())
}

pub fn create_title(html: &mut html_builder::Node) -> AdhanResult<()> {
    let mut head = html.head();
    let _ = head
        .link()
        .attr("rel='stylesheet'")
        .attr("href='current_month.css'");
    writeln!(head.title(), "Adhan - Prayer Time Collector")
        .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
    Ok(())
}

pub fn create_table_body(table: &mut html_builder::Node, month: &[Day]) -> AdhanResult<()> {
    let mut table_body = table.tbody();
    for day in month {
        let mut data_row = table_body.tr();
        writeln!(
            data_row.td().attr("class='tg-baqh'"),
            "{}",
            day.date.format("%A, %d")
        )
        .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
        for prayer in day.prayers {
            writeln!(
                data_row.td().attr("class='tg-baqh'"),
                "{}",
                prayer.time.format("%k:%M")
            )
            .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
        }
    }
    Ok(())
}

pub fn create_table_header(
    table: &mut html_builder::Node,
    current_month: NaiveDate,
) -> AdhanResult<()> {
    let mut table_header = table.thead();
    let mut header_row = table_header.tr();
    writeln!(
        header_row.th().attr("class='tg-baqh'"),
        "{}",
        current_month.format("%b %Y")
    )
    .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
    for elem in ["Fajr", "Dhuhr", "Asr", "Maghrib", "Isha"] {
        writeln!(header_row.th().attr("class='tg-baqh'"), "{}", elem)
            .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
    }
    Ok(())
}

pub fn generate_default_css() -> AdhanResult<()> {
    let css = r#"
    h1 {font-family:Arial, sans-serif;text-align:center;}
.tg {border-collapse:collapse;border-color:#9ABAD9;border-spacing:0;width:100%}
.tg td {background-color:#EBF5FF;border-color:#9ABAD9;border-style:solid;border-width:1px;color:#444;
  font-family:Arial, sans-serif;font-size:14px;overflow:hidden;padding:5px 20px;word-break:normal;text-align:center;}
.tg th {background-color:#409cff;border-color:#9ABAD9;border-style:solid;border-width:1px;color:#fff;
  font-family:Arial, sans-serif;font-size:14px;font-weight:normal;overflow:hidden;padding:5px 20px;word-break:normal;}
.tg tg-baqh{text-align:center;vertical-align:top}
    "#;

    write_file(
        get_user_filepath(),
        std::path::PathBuf::from("current_month.css"),
        css.to_owned(),
    )
}

pub fn generate_template_css() -> AdhanResult<()> {
    println!("Create your own CSS or modify the template at the following path:");

    let css = r#"
h1 {font-family:Arial, sans-serif;text-align:center;}
.tg {}
.tg td {}
.tg th {}
.tg tg-baqh{}
    "#;

    write_file(
        get_user_filepath(),
        std::path::PathBuf::from("current_month.css"),
        css.to_owned(),
    )
}
