//! This crate contains helpers functions for getting user input.
use std::{io, str::FromStr};

/// Ask user to input a number. Loops until a valid digit is entered.
pub fn get_numerical_input<T: FromStr>() -> T {
    loop {
        let input = get_input();
        break match_or_continue!(input.trim().parse::<T>(), "Got non numeric input, try again");
    }
}

/// Ask the user for input.
pub fn get_input() -> String {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Unable to read user input, aborting...");
        std::process::exit(1);
    }

    input
}

/// A helper macro, matching a Result<T, E>, which will continue a loop, if the first parameter is not Ok(T). Can be used with an optional error message.
/// 
/// Usage:
/// ```
/// #[macro_use]
/// extern crate io_utils;
/// use io_utils::match_or_continue;
/// fn main() {
///     let n = loop {
///         break match_or_continue!("2".parse::<u8>(), "Non numerical input");
///     };
///     assert_eq!(n, 2);
/// }
/// ```
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