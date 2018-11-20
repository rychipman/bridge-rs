#![feature(plugin)]
#![feature(custom_attribute)]
#![feature(custom_derive)]
#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
pub extern crate rocket;
#[macro_use]
pub extern crate rocket_contrib;
pub extern crate diesel;
pub extern crate serde_derive;
#[macro_use]
pub extern crate error_chain;
pub extern crate clap;
pub extern crate rand;

pub mod bidding;
pub mod cli;
pub mod game;
pub mod web;
