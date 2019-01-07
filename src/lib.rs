#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
pub extern crate diesel;
#[macro_use]
pub extern crate diesel_migrations;
pub extern crate clap;
#[macro_use]
pub extern crate failure;
#[macro_use]
pub extern crate cursive;
pub extern crate rand;

pub mod bidding;
pub mod cli;
pub mod game;
pub mod tui;
