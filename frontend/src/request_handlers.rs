use std::{sync::{Arc, RwLock}, io::Write};

use utils::{requests::{Request, RequestError}, events::EventType, AppState};
use io_utils::get_numerical_input;
use std::io::stdout;

/// Shorthand for [Result::<T, UserInputError].
type Result<T> = std::result::Result<T, UserInputError>;

/// Error types for invalid user input.
pub enum UserInputError {
    InvalidPauseValue(i64),
    InvalidEventId(u64),
    TaskNotFound(u64),
    MustBePositiveNumber(&'static str),
}

impl std::fmt::Display for UserInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            UserInputError::InvalidPauseValue(val) => format!("Invalid value for pause request: {}", val),
            UserInputError::InvalidEventId(id) => format!("Invalid id for event: {}", id),
            UserInputError::TaskNotFound(id) => format!("No Task forund for id: {}", id),
            UserInputError::MustBePositiveNumber(item) => format!("Value for {} must be positive", item),
        };

        write!(f, "{}", msg)
    }
}

/// Prepare a null request.
pub fn prepare_null() -> Result<()> {
    Ok(())
}

// Prepare a no request
pub fn prepare_no_request() -> Result<()> {
    Ok(())
}

/// Prepare a pauseon request. Only has an effect when using Cpp Ayudame.
pub fn prepare_pause_on_event(buf: &mut [u8]) -> Result<()> {
    println!("Ayudame reacts to pause on the following events:
0:\tNull,
10:\tPreRunTask
14:\tRemoveTask
15:\tWaitOn
16:\tBarrier");
    print!("Enter id of Event: ");
    flush();

    let event_id = get_numerical_input();
    let event = EventType::try_from(event_id).map_err(|_| UserInputError::InvalidEventId(event_id))?;
    let pause_val = get_pause_value()?;

    write_into_buffer(buf, &(event as u64).to_be_bytes(), 2);
    write_into_buffer(buf, &pause_val.to_be_bytes(), 3);

    Ok(())
}

// Prepare a pauseontask request. Only has an effect when using Cpp Ayudame.
pub fn prepare_pause_on_task(buf: &mut [u8], state: &Arc<RwLock<AppState>>) -> Result<()> {
    let task_id = get_task_id(state)?;
    let pause_val = get_pause_value()?;

    write_into_buffer(buf, &task_id.to_be_bytes(), 2);
    write_into_buffer(buf, &pause_val.to_be_bytes(), 3);

    Ok(())
}

/// Currently not implemented and included for completenes.
pub fn prepare_pause_on_function(_buf: &mut [u8]) -> Result<()> {
    eprintln!("Pause on function request not implemted.");

    Ok(())
}

/// Prepare a step request, which will step through the application. Works with Cpp und Rust Ayudame.
pub fn prepare_step(buf: &mut [u8]) -> Result<()> {
    print!("Enter number of steps (must be positive): ");
    flush();

    let step = get_numerical_input::<i64>();
    if step < 0 {
        return Err(UserInputError::MustBePositiveNumber("step request"));
    }

    write_into_buffer(buf, &step.to_be_bytes(), 2);

    Ok(())
}

/// Prepare a breakpoint request. Only has an effect when using Cpp Ayudame.
pub fn prepare_breakpoint(buf: &mut [u8]) -> Result<()> {
    let is_on = get_pause_value()?;

    write_into_buffer(buf, &is_on.to_be_bytes(), 2);

    Ok(())
}

/// Prepeare a blocktask request. Only has an effect when using Cpp Ayudame.
pub fn prepare_block_task(buf: &mut[u8], state: &Arc<RwLock<AppState>>) -> Result<()> {
    let task_id = get_task_id(state)?;
    print!("Indicate if task is blocked: 1 is blocked, else not");
    flush();

    let is_blocked: i64 = get_numerical_input();

    write_into_buffer(buf, &task_id.to_be_bytes(), 2);
    write_into_buffer(buf, &is_blocked.to_be_bytes(), 3);

    Ok(())
}

// Prepare a prioritisetask request. Only has an effect when using Cpp Ayudame.
pub fn prepare_prioritise_task(buf: &mut [u8], state: &Arc<RwLock<AppState>>) -> Result<()> {
    let task_id = get_task_id(state)?;
    print!("Enter priority: ");
    flush();
    let priority: i64 = get_numerical_input();

    write_into_buffer(buf, &task_id.to_be_bytes(), 2);
    write_into_buffer(buf, &priority.to_be_bytes(), 3);

    Ok(())
}

/// Prepare a setnumthreads request. Only works when using Cpp Ayudame.
pub fn prepare_set_num_threads(buf: &mut [u8]) -> Result<()> {
    let n_threads: i64 = get_numerical_input();
    if n_threads < 0 {
        return Err(UserInputError::MustBePositiveNumber("number of threads"));
    }

    write_into_buffer(buf, &n_threads.to_be_bytes(), 2);

    Ok(())
}

/// Prepare a breakattask request. Only works when using Rust Ayudame.
pub fn prepare_break_at_task(buf: &mut [u8], state: &Arc<RwLock<AppState>>) -> Result<()> {
    let task_id = get_task_id(state)?;
    write_into_buffer(buf, &task_id.to_be_bytes(), 2);
    
    Ok(())
}

/// Prepare a unbreak at task request. Only works when using Rust Ayudame.
pub fn prepare_unbreak_at_task(buf: &mut [u8], state: &Arc<RwLock<AppState>>) -> Result<()> {
    let task_id = get_task_id(state)?;
    write_into_buffer(buf, &task_id.to_be_bytes(), 2);

    Ok(())
}

/// Prepare a continue request. Only works when using Rust Ayudame.
pub fn prepare_continue(_buf: &mut [u8]) -> Result<()> {
    Ok(())
}

/// Prepate a break request. Only works when using Rust Ayudame.
pub fn prepare_break(_buf: &mut [u8]) -> Result<()> { 
    Ok(())
}

/// Ask the user to enter the id of an request.
/// 
/// If the id is valid, the Request enum will be returned, otherwise, return an error.
pub fn get_request_type() -> std::result::Result<Request, RequestError> {
    let id = get_numerical_input::<i64>();
    Request::try_from(id)
}

/// Prints a list of possible request types to std::out.
pub fn print_options() {
    println!("Select a request:
 0:\tNull
 1:\tNoRequest
 2:\tPauseOnEvent
 3:\tPauseOnTask
 4:\tPauseOnFunction
 5:\tStep
 6:\tBreakpoint
 7:\tBlockTask
 8:\tPrioritiseTask
 9:\tSetNumThreads
10:\tContinue
11:\tBreak
12:\tBreakAtTask
")
}

/// Write the request into a buffer, which can be sent via a socket.
pub fn write_request(buf: &mut [u8], request: &Request) {
    write_into_buffer(buf, &(*request as u64).to_be_bytes(), 1);
}

/// Write bytes into buf at the specified index.
#[inline(always)]
fn write_into_buffer(buf: &mut [u8], bytes: &[u8], index: usize) {
    bytes.into_iter().enumerate().for_each(|(i, n)| buf[8 * index + i] = *n);
}

/// Ask the user to enter a pause value, which is needed for some requests.
fn get_pause_value() -> Result<u64> {
    print!("Enter pause value (1 on, 0 off): ");
    flush();

    let pause_val = get_numerical_input();
    if pause_val != 0 && pause_val != 1 {
        return Err(UserInputError::InvalidEventId(pause_val));
    }
    Ok(pause_val)
}

/// Prompts the user to enter a task id.
fn get_task_id(state: &Arc<RwLock<AppState>>) -> Result<u64> {
    print!("Enter task id: ");
    flush();

    let task_id = get_numerical_input();
    {
        let s = state.read().unwrap();
        if s.get_task(task_id).is_none() {
            return Err(UserInputError::TaskNotFound(task_id));
        }
    }
    Ok(task_id)
}

/// Flush a buffer to std::out.
fn flush() {
    let _ = stdout().flush();
}