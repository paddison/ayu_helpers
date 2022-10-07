mod helper_macros;
pub mod requests;

use std::{
    fmt::Write,
    os::raw::c_char,
    fmt::Display, sync::{Arc, Weak, Mutex},
};

pub mod events;

/// # State of the Application
/// 
/// Includes all the tasks and functions added so far.
/// `task_id_count`: Counter to create unique task ids 
#[derive(Debug)]
pub struct AppState {
    pub is_pre_init: bool,
    pub is_init: bool,
    tasks: Vec<Arc<Task>>,
    functions: Vec<Arc<Function>>,
}

impl AppState {

    /// Creates a new empty State
    pub fn new() -> Self {
        AppState {
            is_pre_init: false,
            is_init: false,
            tasks: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn list_functions(&self) {
        for f in &self.functions {
            println!("{}", f)
        }
    }

    pub fn create_task_id(&self) -> u64 {
        self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    /// Creates a new function from a user provided name
    /// Returns None if the provided name contained non ASCII chars
    pub fn create_function(&mut self, id: u64, name: String) -> Option<Arc<Function>> {
        // create a new id (this only works if we never delete a created label)
        match name.trim() {
            "" => {
                let f: Arc<Function> = Arc::new(id.into());
                self.functions.push(Arc::clone(&f));
                Some(f)
            },
            _ => match Function::new(id, name.trim().to_string()) {
                Ok(f) => { 
                    let f = Arc::new(f);
                    self.functions.push(Arc::clone(&f));
                    Some(f)
                },
                Err(_) => None,
            },
        }
    }

    pub fn create_function_id(&self) -> u64 {
        self.functions.len() as u64
    }

    pub fn list_tasks(&self) {
        for t in &self.tasks {
            println!("{}", t);
        }
    }

    pub fn does_task_exist(&self, id: u64) -> bool {
        self.tasks.iter().position(|t| t.id == id).is_some()
    }

    pub fn get_task(&self, id: u64) -> Option<&Arc<Task>> {
        self.tasks.iter()
            .position(|t| t.id == id)
            .and_then(|idx| self.tasks.get(idx))
    }

    fn get_dependencies(&self) -> Vec<(u64, u64)> {
        let mut dependencies = Vec::new();
        for parent in &self.tasks {
            for child_ptr in parent.children.lock().unwrap().iter() {
                if let Some(child) = child_ptr.upgrade() {
                    dependencies.push((parent.id, child.id))
                }
            }
        }

        dependencies
    }

    /// Create a new task
    pub fn create_task(&mut self, id: u64, is_critical: bool, function_id: Option<u64>, thread_id: u64) -> Result<Arc<Task>, &str> {
        
        // check if function for provided id exists
        let function = match function_id {
            Some(id) => {
                let id = self.functions.get(id as usize).ok_or("Provided id not in list.")?;
                Some(Arc::downgrade(id))
            },
            None => None,
        };
        
        let task = Arc::new(Task {
            id,
            thread_id,
            function,
            is_critical,
            parents: Mutex::new(Vec::new()),
            children: Mutex::new(Vec::new()),
        });

        self.tasks.push(Arc::clone(&task));
        Ok(task)
    }

    pub fn delete_task(&mut self, task_id: u64) -> Option<()> {

        self.tasks.iter()
            .position(|t| t.id == task_id)
            .and_then(|idx| { self.tasks.remove(idx); Some(()) })
    }

    pub fn add_dependency(&mut self, parent_id: u64, child_id: u64) -> Option<()> {
        let parent = self.get_task(parent_id)?;
        let child = self.get_task(child_id)?;

        {
            let mut children = parent.children.lock().unwrap();
            children.push(Arc::downgrade(child));
        }

        {
            let mut parents = child.parents.lock().unwrap();
            parents.push(Arc::downgrade(parent));
        }

        Some(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut task_string = String::new();
        for t in &self.tasks {
            let _ = write!(task_string, "\n\t\t{}", t);
        }

        let mut function_string = String::new();
        for f in &self.functions {
            let _ = write!(function_string, "\n\t\t{}", f);
        }

        let mut dependencies_string = String::new();
        for d in self.get_dependencies() {
            let _ = write!(dependencies_string, "\n\t\t(P: {}, C: {})", d.0, d.1);
        }

        write!(f, "Current State:\n\tPreInitialized: {}\n\tInitialized: {}\n\tTasks: {}\n\tFunctions/Labels: {}\n\tDependencies: {}", self.is_pre_init, self.is_init, task_string, function_string, dependencies_string)
    }
}

#[derive(Debug)]
pub struct Task {
    id: u64,
    thread_id: u64,
    function: Option<Weak<Function>>,
    is_critical: bool,
    parents: Mutex<Vec<Weak<Task>>>,
    children: Mutex<Vec<Weak<Task>>>,
}

impl Task {
    pub fn into_raw_parts(&self) -> (u64, u64, u64, u64) {
        let function_id = self.function
                            .as_ref()
                            .and_then(|f| f.upgrade())
                            .map_or(self.id, |f| f.id);

        (self.id, function_id, if self.is_critical { 1 } else { 0 }, self.thread_id)
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let f_label = self.function
                            .as_ref()
                            .and_then(|f| f.upgrade())
                            .map_or("None".to_string(), |f| f.name.clone());

        let string = format!("{}: label = {}, is_critical = {}, thread_id = {}", self.id, f_label, self.is_critical, self.thread_id);
        write!(f, "{}", string)
    }
}

// impl Eq for Task {}

impl From<u64> for Task {
    fn from(id: u64) -> Self {
        Task {
            id,
            thread_id: 0,
            function: Some(Arc::downgrade(&Arc::new(0.into()))),
            is_critical: false,
            parents: Mutex::new(Vec::new()),
            children: Mutex::new(Vec::new()),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub id: u64,
    pub name: String,
}

impl Function {
    pub fn new(id: u64, mut name: String) -> Result<Self, &'static str> {
        // make sure string is valid ascii
        // let mut name = name.to_owned();
        if !name.is_ascii() {
            return Err("string contains non ascii characters");
        }
        
        // add null byte for c string
        name += "\0";

        Ok(Self { id, name })
    }

    pub fn into_raw_parts(&self) -> (u64, *mut c_char) {
        (self.id, self.name.as_ptr() as *mut c_char)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}

impl From<u64> for Function {
    fn from(id: u64) -> Self {
        Function {
            id,
            name: format!("default_function_{id}\0"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AppState, Function};

    #[test]
    fn function_new_is_ok() {
        let f = Function::new(0, String::from("function"));
        assert!(f.is_ok());
    }

    #[test]
    fn function_new_is_err() {
        let f = Function::new(0, String::from("功能"));
        assert!(f.is_err());
    }

    #[test]
    fn function_prepare_for_sending() {
        let f = Function::new(0, String::from("function0")).unwrap();
        let (_, name) = f.into_raw_parts();

        unsafe {
            assert_eq!(*name, 102); // f
            assert_eq!(*name.add(1), 117); // u
            assert_eq!(*name.add(2), 110); // n
            assert_eq!(*name.add(3), 99); // c
            assert_eq!(*name.add(4), 116); // t
            assert_eq!(*name.add(5), 105); // i
            assert_eq!(*name.add(6), 111); // o
            assert_eq!(*name.add(7), 110); // n
            assert_eq!(*name.add(8), 48); // 0
            assert_eq!(*name.add(9), 0); // \0
        }
    }

    #[test]
    fn app_state_create_function() {
        let mut state = AppState::new();
        assert_eq!(state.functions.len(), 0);

        state.create_function(0, "functino".to_string());
        assert_eq!(state.functions.len(), 1);

        let result = state.functions.get(0);
        assert!(result.is_some());

        let f = result.unwrap();
        assert_eq!(f.name, "functino\0".to_string());

        state.create_function(1, "funco".to_string());
        assert_eq!(state.functions.len(), 2);

        let result = state.functions.get(1);
        assert!(result.is_some());

        let f = result.unwrap();
        assert_eq!(f.name, "funco\0".to_string());
    }

    #[test]
    fn app_state_create_task() {
        let mut state = AppState::new();

        assert!(state.create_task(0, false, Some(0), 1).is_err());

        let _ = state.create_function(0, "f1".to_string());

        assert!(state.create_task(0, false, Some(0), 0).is_ok());
    }
}
