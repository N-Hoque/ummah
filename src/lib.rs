//! # Adhan
//!
//! ## Overview
//!
//! Adhan is an application for obtaining the prayer times from [www.salahtimes.com/uk](www.salahtimes.com/uk)
//!
//! It provides support for settings prayer time calculations for
//! adherents of different schools of thought.
//!
//! ## Installation
//!
//! Adhan can be installed using `cargo install adhan`. Using the `--profile performance` installs a highly optimised binary.
//!
//! ## Usage
//!
//! Simply run `adhan` in the terminal to get the prayer times for the current month.
//!
//! Run `adhan -t` to get the times for today.

pub mod argparser;
pub mod core;
pub(crate) mod request_parser;
pub mod time;
pub mod types;
