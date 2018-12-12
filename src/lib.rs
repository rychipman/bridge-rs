#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
pub extern crate diesel;
pub extern crate clap;
pub extern crate rand;

pub mod bidding;
pub mod cli;
pub mod game;
