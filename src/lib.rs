//! # Ummah
//!
//! ## Overview
//!
//! Ummah is a library for obtaining the prayer times from [www.salahtimes.com/uk](www.salahtimes.com/uk)
//!
//! It provides support for settings prayer time calculations for
//! different schools of thought.

pub mod argparser;
pub mod core;
pub(crate) mod request_parser;
pub mod time;
pub mod types;
