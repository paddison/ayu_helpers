use std::io::Read;
use std::net::TcpStream;
use std::sync::{RwLock, Arc};

use utils::AppState;
use utils::events::{self, Event, EventType, EventError};

pub enum EventResult {
    Exit,
    Success,
    Fail, // maybe add this??
}

pub fn handle_event(buf: &[u8], mut state_lock: &mut Arc<RwLock<AppState>>, stream: &mut TcpStream) -> Result<EventResult, EventError> {
    let result = match Event::try_from(buf)? {
        Event::PreInit { rt, pid } => handle_pre_init(rt, pid, &mut state_lock),
        Event::Init { n_threads } => handle_init(n_threads, &mut state_lock),
        Event::AddTask { task_id, func_id, priority, scope_id } => handle_add_task(task_id, func_id, priority, scope_id, &mut state_lock),
        Event::RegisterFunction { func_id, string_len } => {
            // try to read the name from the stream
            let name = if string_len > 0 {
                let mut buf = [0; 128];
                let bytes_read = stream.read(&mut buf).unwrap_or(0);
                if bytes_read != string_len {
                    eprintln!("String Length doesn't match bytes read, defaulting to empty function name.");
                    String::new()
                } else {
                    events::read_function_name_from_buffer(&buf[..string_len])
                }
            } else {
                String::new()
            };
            handle_register_function(func_id, name, string_len, &mut state_lock)
        },
        Event::AddDependency { to_id, from_id, memaddr, orig_memaddr } => handle_add_dependency(to_id, from_id, memaddr, orig_memaddr, &mut state_lock),
        Event::AddTaskToQueue { task_id, thread_id } => handle_add_task_to_queue(task_id, thread_id, &mut state_lock),
        Event::PreRunTask { task_id, thread_id } => handle_pre_run_task(task_id, thread_id, &mut state_lock),
        Event::RunTask { task_id } => handle_run_task(task_id, &mut state_lock),
        Event::PostRunTask { task_id } => handle_post_run_task(task_id, &mut state_lock),
        Event::RemoveTask { task_id } => handle_remove_task(task_id, &mut state_lock),
        Event::Barrier => handle_barrier(&mut state_lock),
        Event::WaitOn { task_id } => handle_wait_on(task_id, &mut state_lock),
        Event::Finish => handle_finish(&mut state_lock),
    }; 

    // println!("{}\n", state_lock.read().unwrap());

    Ok(result)
}

fn handle_pre_init(rt: u64, pid: u64, state_lock: &RwLock<AppState>) -> EventResult {
    println!("Got PreInit event, rt: {rt}, pid: {pid}");
    if let Ok(mut state) = state_lock.write() {
        state.is_pre_init = true;
    }

    EventResult::Success
}

fn handle_init(n_threads: u64, state_lock: &RwLock<AppState>) -> EventResult {
    println!("Got Init event, n_threads: {n_threads}");
    if let Ok(mut state) = state_lock.write() {
        state.is_init = true;
    }

    EventResult::Success
}

fn handle_add_task(task_id: u64, func_id: u64, priority: u64, scope_id: u64, state_lock: &RwLock<AppState>) -> EventResult {
    println!("Got AddTask Event.");
    let function_id = if func_id == 0 { Some(func_id) } else { None };
    let is_critical = if priority > 0 { true } else { false };
    if let Ok(mut state) = state_lock.write() {
        let _ = state.create_task(task_id, is_critical, function_id, scope_id);
    }

    EventResult::Success
}

fn handle_register_function(func_id: u64, name: String, string_len: usize, state_lock: &RwLock<AppState>) -> EventResult {
    println!("Got RegisterFunction event");
    if name.len() != string_len {
        eprintln!("name length({}) not matching provided string_len ({}).", name.len(), string_len)
    }

    if let Ok(mut state) = state_lock.write() {
        state.create_function(func_id, name);
    }

    EventResult::Success
}

fn handle_add_dependency(to_id: u64, from_id: u64, memaddr: u64, orig_memaddr: u64, state_lock: &RwLock<AppState>) -> EventResult {
    println!("Got AddDependency event, memadd: {:x}, orig_memaddr: {:x}", memaddr, orig_memaddr);
    if let Ok(mut state) = state_lock.write() {
        state.add_dependency(to_id, from_id);
    }

    EventResult::Success
}

fn handle_add_task_to_queue(task_id: u64, thread_id: u64, _state: &RwLock<AppState>) -> EventResult {
    println!("Got add AddTaskToQueue event, task_id: {task_id}, thread_id: {thread_id}");
    
    EventResult::Success
}

fn handle_pre_run_task(task_id: u64, thread_id: u64, _state: &RwLock<AppState>) -> EventResult {
    println!("Got PreRunTask event, task_id: {task_id}, thread_id: {thread_id}");
    
    EventResult::Success
}

fn handle_run_task(task_id: u64, _state: &RwLock<AppState>) -> EventResult {
    println!("Got RunTask event, task_id: {task_id}");

    EventResult::Success
}

fn handle_post_run_task(task_id: u64, _state: &RwLock<AppState>) -> EventResult {
    println!("Got PostRunTask event, task_id: {task_id} ");

    EventResult::Success
}

fn handle_remove_task(task_id: u64, state_lock: &RwLock<AppState>) -> EventResult {
    println!("Got RemoveTask event, task_id: {task_id}");
    if let Ok(mut state) = state_lock.write() {
        state.delete_task(task_id);
    }
    EventResult::Success
}

fn handle_barrier(_state: &RwLock<AppState>) -> EventResult {
    println!("Got Barrier event.");

    EventResult::Success
}

fn handle_wait_on(task_id: u64,_state: &RwLock<AppState>) -> EventResult {
    println!("Got WaitOn event, task_id: {task_id}");

    EventResult::Success
}

fn handle_finish(_state: &RwLock<AppState>) -> EventResult {
    println!("Got finish event, exiting...");

    EventResult::Exit
}