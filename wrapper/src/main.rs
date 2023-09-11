//! A small wrapper around Ayudame for debugging.
//! It makes it possible to connect to a frontend and create any kind of event.
//! After starting the wrapper, it will wait for a frontend to connect.
//! Note that AYU_PORT must be set to a free port, before starting the wrapper.
//! 
//! Usage: AYU_PORT=XXXX cargo run --release
mod subcommands;
use std::{ env, fs };


use subcommands::custom;
use subcommands::file;
use subcommands::generate;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 3 {
        if args[1] == "file".to_string() {
            if let Err(e) = file::from_file(&args[2]) {
                eprintln!("{}", e.to_string());
            }

        } 
    } else {
        custom::run_custom_events();
    }
    //gg::run_generate_graph();
}

