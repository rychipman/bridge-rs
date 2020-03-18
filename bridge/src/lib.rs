#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
pub extern crate diesel;
#[macro_use]
pub extern crate diesel_migrations;
pub extern crate clap;
#[macro_use]
pub extern crate failure;
pub extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod bidding;
pub mod cli;
pub mod game;
