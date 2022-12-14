use std::{io, str::FromStr};

pub fn get_numerical_input<T: FromStr>() -> T {
    loop {
        let input = get_input();
        break match_or_continue!(input.trim().parse::<T>(), "Got non numeric input, try again");
    }
}

pub fn get_input() -> String {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Unable to read user input, aborting...");
        std::process::exit(1);
    }

    input
}

#[macro_export]
macro_rules! match_or_continue {
    ($func:expr, $msg:expr) => {
        match $func {
            Ok(val) => val,
            Err(_) => {
                eprintln!("{}", $msg);
                continue;
            }
        }   
    };
    ($func:expr) => {
        match $func {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }   
    };
}