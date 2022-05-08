use crate::{
    day::Day,
    types::{AdhanError, AdhanResult},
};

use chrono::NaiveDate;
use html_builder::Html5;

use std::{fmt::Write, path::PathBuf};

use super::fs::{get_user_filepath, write_file};

static CURRENT_HTML: &str = "current_month.html";

pub struct TimetableGenerator {
    generate_css: bool,
}

impl TimetableGenerator {
    pub fn new(generate_css: bool) -> Self {
        Self { generate_css }
    }

    /// Creates an HTML page for the prayer timetable
    pub fn generate(&self, month: &[Day]) -> AdhanResult<()> {
        let mut document = html_builder::Buffer::new();

        let mut html = document.html().attr("lang=en-gb");

        TimetableGenerator::create_title(&mut html)?;
        TimetableGenerator::create_table(&mut html, month)?;

        let final_document = document.finish();

        let user_path = get_user_filepath();

        write_file(
            &user_path,
            &PathBuf::from(CURRENT_HTML),
            final_document.as_bytes(),
        )?;

        if self.generate_css {
            TimetableGenerator::generate_default_css()?;
        } else {
            TimetableGenerator::generate_template_css()?;
        }

        Ok(())
    }

    fn create_table(html: &mut html_builder::Node, month: &[Day]) -> AdhanResult<()> {
        let mut body = html.body();

        writeln!(body.h1(), "Adhan").map_err(|x| AdhanError::Unknown(Box::new(x)))?;

        let mut table = body.table().attr("class='tg'");
        TimetableGenerator::create_table_header(&mut table, month[0].get_date())?;
        TimetableGenerator::create_table_body(&mut table, month)?;
        Ok(())
    }

    fn create_title(html: &mut html_builder::Node) -> AdhanResult<()> {
        let mut head = html.head();
        let _ = head
            .link()
            .attr("rel='stylesheet'")
            .attr("href='current_month.css'");
        writeln!(head.title(), "Adhan - Prayer Time Collector")
            .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
        Ok(())
    }

    fn create_table_body(table: &mut html_builder::Node, month: &[Day]) -> AdhanResult<()> {
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

    fn create_table_header(
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

    fn generate_default_css() -> AdhanResult<()> {
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
            css.as_bytes(),
        )
    }

    fn generate_template_css() -> AdhanResult<()> {
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
            css.as_bytes(),
        )
    }
}