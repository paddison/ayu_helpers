//! A small wrapper around Ayudame for debugging.
//! It makes it possible to connect to a frontend and create any kind of event.
//! After starting the wrapper, it will wait for a frontend to connect.
//! Note that AYU_PORT must be set to a free port, before starting the wrapper.
//! 
//! Usage: AYU_PORT=XXXX cargo run --release
mod subcommands;
use subcommands::custom_events as ce;
use subcommands::generate_graph as gg;


fn main() {
    //gg::run_generate_graph();
    ce::run_custom_events();
}