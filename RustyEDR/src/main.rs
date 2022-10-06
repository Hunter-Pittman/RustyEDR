// Modules
mod webserver_lib;
mod data_populate;
mod database_manager;
mod process_monitor;

use std::fs::File;
use std::io::prelude::*;
use webserver_lib::*;
use futures::prelude::*;
use std::path::Path;
use std::env;
use sqlite;
use data_populate::*;
use database_manager::*;
use process_monitor::*;

#[tokio::main]
async fn main() {
    // Initial environment checks
    let pwd = env::current_dir().unwrap();
    let full_path = pwd.display().to_string() + "\\rustyedr.db";
    let db_exists = Path::new(&full_path).exists();

    if db_exists == false {
        create_database(full_path);
    }

    process_info();

}