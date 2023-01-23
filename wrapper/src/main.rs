/// A debugging application to test out ayu_events functions

// Should be able to send any event to ayudame at any time,
// CLI app
// create task ids, function ids, etc with counters

use std::fmt::Display;
use std::convert::TryFrom;
use utils::AppState;
use utils::events::EventType;
use io_utils::{match_or_continue, get_numerical_input, get_input};
use ayudame_core_rs::ayu_events::*;

const DUMMY_MEMADDR: u64 = 0xffee0000;

const PARSE_UNSIGNED_ERROR_MSG: &str = "Invalid input, must be positive numeric";

type Result<T> = std::result::Result<T, UserInputError>;

enum Command {
    AddTask,
    PrintState,
}

#[derive(Debug)]
enum UserInputError {
    TaskIdNotFound(u64),
    AlreadyInitialized(&'static str),
    InvalidFunctionName(String),
    SameTaskDependency,
    EventNotImplemented(EventType)
}

impl Display for UserInputError {
    
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use UserInputError::*;
        
        let msg = match self {
            AlreadyInitialized(init) => format!("{} should only be called once. Will not emit event.", init),
            TaskIdNotFound(id) => format!("Task with id: {} not found.", id),
            InvalidFunctionName(name) => format!("Invalid Name: {}. Can only contain ASCII characters", name.trim()),
            SameTaskDependency => "Parent and Child cannot be the same Task.".to_string(),
            EventNotImplemented(event) => format!("Event {:?} not implemented.", event),
        };
        write!(f, "Error while reading input:\n\t{}", msg)
    }
}

impl std::error::Error for UserInputError { }


fn main() {
    // create event loop
    let mut state = AppState::default();

    // let _ = create_pre_init(&mut state);
    // let _ = create_init(&mut state);
    
    loop {
        match ask_for_command() {
            Command::AddTask => {
                print_event_types();

                if let Err(e) = handle_user_input(&mut state) {
                    eprintln!("{}", e);
                }
            },
            Command::PrintState => println!("{}", state),
        }
    }
}

fn ask_for_command() -> Command {
    println!("Options:\n\t(a)dd new event\n\t(p)rint current state");
    loop {
        break match get_input().trim() {
            "a" => Command::AddTask,
            "p" => Command::PrintState,
            invalid => {
                eprintln!("Invalid Option: {}, try again", invalid);
                continue;
            },
        }   
    }
}

fn print_event_types() {
    let options_str = 
    "Event Types: 
        0.  Null (not implemented)
        1.  PreInit
        2.  Init
        3.  Finish
        4.  RegisterFunction
        5.  AddTask
        6.  AddHiddenTask (not implemented)
        7.  AddDependency
        8.  AddTaskToQueue
        9.  AddPreSelectTask (not implemented)
        10. PreRunTask
        11. RunTask
        12. PostRunTask
        13. RunTaskFailed (not implemented)
        14. RemoveTask
        15. WaitOn
        16. Barrier
        17. AddWaitOnTask (not implemented)
        ";
        println!("{options_str}");
}

pub fn get_event_type() -> EventType { 
    println!("Enter index of Event to send: ");
    
    loop {
        let n = get_numerical_input::<u64>();

        break match_or_continue!(EventType::try_from(n), "Got Invalid Index, try again");
    } 
}

fn handle_user_input(state: &mut AppState) -> Result<()> {
    match get_event_type() {
        EventType::PreInit => create_pre_init(state),
        EventType::Init => create_init(state),
        EventType::AddTask => create_add_task(state),
        EventType::RegisterFunction => create_register_function(state),
        EventType::AddDependency => create_add_dependency(state),
        EventType::AddTaskToQueue => create_add_task_to_queue(state),
        EventType::PreRunTask => create_pre_run_task(state),
        EventType::RunTask => create_run_task(state),
        EventType::PostRunTask => create_post_run_task(state),
        EventType::RemoveTask => create_remove_task(state),
        EventType::Barrier => create_barrier(),
        EventType::WaitOn => create_wait_on(state),
        EventType::Finish => create_finish(),
        event => Err(UserInputError::EventNotImplemented(event))
    }
}

fn create_pre_init(state: &mut AppState) -> Result<()> {
    if state.is_pre_init {
        return Err(UserInputError::AlreadyInitialized("PreInit"));
    }
    ayu_event_preinit(0); 

    state.is_pre_init = true;
    Ok(())
}

fn create_init(state: &mut AppState) -> Result<()> {
    if state.is_init {
        return Err(UserInputError::AlreadyInitialized("Init"));
    }
    ayu_event_init(2);

    state.is_init = true;

    Ok(())
}

fn create_add_task(state: &mut AppState) -> Result<()>{
    // TODO: Return with error on wrong input
    println!("Specify Task to add: (leave empty for default values");

    println!("Is task critical (default is false)? (y/n)");
    let is_critical = loop {
        match get_input().trim() {
            "y" => break true,
            "n" => break false,
            "" => break false,
            invalid => eprintln!("Invalid option: {}", invalid),
        }
    };

    println!("Enter thread id: (default is 0)");
    let thread_id = loop {
        break match get_input().trim() {
            "" => 0,
            n => match_or_continue!(n.parse::<u64>(), PARSE_UNSIGNED_ERROR_MSG),
        };
    };

    println!("Choose a label for task: ");
    state.list_functions();
    let task = loop {
        let function_id = match get_input().trim() {
            "" => None,
            input => Some(match_or_continue!(input.parse::<u64>(), PARSE_UNSIGNED_ERROR_MSG)),
        };
        let task_id = state.create_task_id();
        break match_or_continue!(state.create_task(task_id, is_critical, function_id, thread_id), "Function with provided id not found");
    };

    let (task_id, func_id, priority, scope_id) = task.into_raw_parts();

    ayu_event_addtask(task_id, func_id, priority, scope_id);

    Ok(())
}

// 
fn create_register_function(state: &mut AppState) -> Result<()> {
    println!("Enter a name for function (empty for default)");
    let name = get_input();
    let id = state.create_function_id();
    let function = state.create_function(id, name.clone()).ok_or(UserInputError::InvalidFunctionName(name))?;

    let (id, name) = function.into_raw_parts();

    ayu_event_registerfunction(id, name);

    Ok(())
}

fn create_add_dependency(state: &mut AppState) -> Result<()> {
    state.list_tasks();

    println!("Enter parent, then child id");

    let parent_id = specify_task_id(state)?;

    let child_id = specify_task_id(state)?;

    if child_id == parent_id {
        return Err(UserInputError::SameTaskDependency);
    } 

    let memaddr = DUMMY_MEMADDR | parent_id;
    let orig_memaddr = DUMMY_MEMADDR | child_id;
    
    state.add_dependency(parent_id, child_id);

    ayu_event_adddependency(parent_id, child_id, memaddr, orig_memaddr);
    Ok(())
}

fn create_add_task_to_queue(state: & AppState) -> Result<()> {
    state.list_tasks();
    
    let task_id = get_numerical_input();
    let (_, _, _, scope_id) = state.get_task(task_id).ok_or(UserInputError::TaskIdNotFound(task_id))?.into_raw_parts();

    ayu_event_addtasktoqueue(task_id, scope_id);

    Ok(())
}

fn create_pre_run_task(state: &AppState) -> Result<()> {
    state.list_tasks();
    let task_id = get_numerical_input();

    let (_, _, _, scope_id) = state.get_task(task_id).ok_or(UserInputError::TaskIdNotFound(task_id))?.into_raw_parts();

    ayu_event_preruntask(task_id, scope_id);

    Ok(())
}

fn create_run_task(state: &AppState) -> Result<()> {
    state.list_tasks();
    let task_id = specify_task_id(state)?;

    ayu_event_runtask(task_id);

    Ok(())
}

fn create_post_run_task(state: &AppState) -> Result<()> {
    state.list_tasks();
    let task_id = specify_task_id(state)?;

    ayu_event_postruntask(task_id);

    Ok(())
}

fn create_remove_task(state: &mut AppState) -> Result<()> {
    state.list_tasks();
    let task_id = specify_task_id(state)?;

    state.delete_task(task_id).ok_or(UserInputError::TaskIdNotFound(task_id))?;

    ayu_event_removetask(task_id);

    Ok(())
}

fn create_barrier() -> Result<()> {
    ayu_event_barrier();

    Ok(())
}

fn create_wait_on(state: &AppState) -> Result<()> {
    state.list_tasks();
    let task_id = specify_task_id(state)?;

    ayu_event_waiton(task_id);

    Ok(())
}

fn create_finish() -> Result<()> {
    ayu_event_finish();

    Ok(())
}

fn specify_task_id(state: &AppState) -> Result<u64> {
    println!("Select Task: ");
    let id = get_numerical_input::<u64>() as u64;
    if !state.does_task_exist(id) {
        Err(UserInputError::TaskIdNotFound(id))
    } else {
        Ok(id)
    }
}