use crate::{
    day::Day,
    types::{AdhanError, AdhanResult},
};

use html_builder::Html5;

use std::fmt::Write;

pub fn create_table(html: &mut html_builder::Node, month: &[Day]) -> AdhanResult<()> {
    let mut body = html.body();
    let mut table = body.table();
    create_table_header(&mut table)?;
    create_table_body(&mut table, month)?;
    Ok(())
}

pub fn create_title(html: &mut html_builder::Node) -> AdhanResult<()> {
    let mut head = html.head();
    writeln!(head.title(), "Adhan - Prayer Time Collector")
        .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
    Ok(())
}

pub fn create_table_body(table: &mut html_builder::Node, month: &[Day]) -> AdhanResult<()> {
    let mut table_body = table.tbody();
    for day in month {
        let mut data_row = table_body.tr();
        writeln!(data_row.td(), "{}", day.date).map_err(|x| AdhanError::Unknown(Box::new(x)))?;
        writeln!(data_row.td(), "{}", day.prayers[0].time)
            .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
        writeln!(data_row.td(), "{}", day.prayers[1].time)
            .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
        writeln!(data_row.td(), "{}", day.prayers[2].time)
            .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
        writeln!(data_row.td(), "{}", day.prayers[3].time)
            .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
        writeln!(data_row.td(), "{}", day.prayers[4].time)
            .map_err(|x| AdhanError::Unknown(Box::new(x)))?;
    }
    Ok(())
}

pub fn create_table_header(table: &mut html_builder::Node) -> AdhanResult<()> {
    let mut table_header = table.thead();
    let mut header_row = table_header.tr();
    for elem in ["Date", "Fajr", "Dhuhr", "Asr", "Maghrib", "Isha"] {
        writeln!(header_row.th(), "{}", elem).map_err(|x| AdhanError::Unknown(Box::new(x)))?;
    }
    Ok(())
}
