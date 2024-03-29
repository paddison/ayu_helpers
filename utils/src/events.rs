use std::{fmt::Display, time::Duration};

#[derive(Debug)]
pub enum Event {
    PreInit{ rt: u64, pid: u64 },
    Init{ n_threads: u64 },
    Finish,
    RegisterFunction{ func_id: u64, string_len: usize },
    AddTask{ task_id: u64, func_id: u64, priority: u64, scope_id: u64 },
    AddDependency{ to_id: u64, from_id: u64, memaddr: u64, orig_memaddr: u64 },
    AddTaskToQueue{ task_id: u64, thread_id: u64 },
    PreRunTask{ task_id: u64, thread_id: u64 },
    RunTask{ task_id: u64 },
    PostRunTask{ task_id: u64 },
    RemoveTask{ task_id: u64 },
    WaitOn{ task_id: u64 },
    Barrier,
}

#[derive(Debug)]
pub enum EventError {
    InvalidId(u64),
    NotImplemented(EventType),
    EventBufferTooShort(usize),
    BufferUnevenByteBoundary,
}

impl Display for EventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            EventError::InvalidId(id) => format!("Invalid Event id: {}", id),
            EventError::NotImplemented(e_type) => format!("Event Type not implemented: {:?}", e_type),
            EventError::EventBufferTooShort(size) => format!("Buffer too short, is {} bytes. Needs to be at least 64 bytes", size),
            EventError::BufferUnevenByteBoundary => String::from("Buffer length needs to be on even byte boundary (multiple of 8).")
        };

        write!(f, "{}", msg)
    }
}


// TODO implement error type for enum creation failure
impl TryFrom<&[u8]> for Event {
    type Error = EventError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

        if buf.len() < 64 {
            return Err(EventError::EventBufferTooShort(buf.len()));
        }

        let u64_buf = u8_buf_to_u64_buf(&buf[..64])?;

        // values that are always the same for each event
        let rt = u64_buf[0];
        let task_id = u64_buf[1];
        let event_id = u64_buf[2];

        let event = match EventType::try_from(event_id)? {
            EventType::PreInit => Self::PreInit { rt, pid:  task_id }, // for PreInit events, the pid is stored at the index where the task_id is (index 1)
            EventType::Init => Self::Init { n_threads: u64_buf[3] },
            EventType::AddTask => Self::AddTask { 
                task_id, 
                func_id: u64_buf[3], 
                priority: u64_buf[4], 
                scope_id: u64_buf[6] 
            },
            EventType::RegisterFunction =>  {
                // register function sends two buffers:
                // one is the normal data(the first one)
                // the other contains the function name
                let string_len = u64_buf[3] as usize;

                Self::RegisterFunction { 
                func_id: u64_buf[4], 
                string_len
                }
            }, 
            EventType::AddDependency => Self::AddDependency { 
                to_id: task_id, 
                from_id: u64_buf[3], 
                memaddr: u64_buf[4], 
                orig_memaddr: u64_buf[5] 
            },
            EventType::AddTaskToQueue => Self::AddTaskToQueue { task_id, thread_id: u64_buf[3] },
            EventType::PreRunTask => Self::PreRunTask { task_id, thread_id: u64_buf[3] },
            EventType::RunTask => Self::RunTask { task_id },
            EventType::PostRunTask => Self::PostRunTask { task_id },
            EventType::RemoveTask => Self::RemoveTask { task_id },
            EventType::Barrier => Self::Barrier,
            EventType::WaitOn => Self::WaitOn { task_id },
            EventType::Finish => Self::Finish,
            e_type => return Err(EventError::NotImplemented(e_type)),
        };

        Ok(event)
    }
}

/// # Ayudame Event Types
/// 
/// These are all the Events that get emitted by Ayudame.
#[derive(Debug, PartialEq)]
pub enum EventType {
    Null,
    PreInit,
    Init,
    Finish,
    RegisterFunction,
    AddTask,
    AddHiddenTask,
    AddDependency,
    AddTaskToQueue,
    AddPreSelectTask,
    PreRunTask,
    RunTask,
    PostRunTask,
    RunTaskFailed,
    RemoveTask,
    WaitOn,
    Barrier,
    AddWaitOnTask,
}

impl TryFrom<&[u8]> for EventType {
    type Error = EventError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        use EventType::*;
        if buf.len() < 64 {
            return Err(EventError::EventBufferTooShort(buf.len()));
        }

        let mut id_buf = [0u8; 8];
        id_buf.copy_from_slice(&buf[16..24]);

        let id = u64::from_be_bytes(id_buf);

        let event = match id {
            0 => Null,
            1 => PreInit,
            2 => Init,
            3 => Finish,
            4 => RegisterFunction,
            5 => AddTask,
            6 => AddHiddenTask,
            7 => AddDependency,
            8 => AddTaskToQueue,
            9 => AddPreSelectTask,
            10 => PreRunTask,
            11 => RunTask,
            12 => PostRunTask,
            13 => RunTaskFailed,
            14 => RemoveTask,
            15 => WaitOn,
            16 => Barrier,
            17 => AddWaitOnTask,
            id => return Err(EventError::InvalidId(id)),
        };

        Ok(event)
    }
}

impl TryFrom<u64> for EventType {
    type Error = EventError;
    
    fn try_from(event_id: u64) -> Result<Self, Self::Error> {
        use EventType::*;

        let event = match event_id {
            0 => Null,
            1 => PreInit,
            2 => Init,
            3 => Finish,
            4 => RegisterFunction,
            5 => AddTask,
            6 => AddHiddenTask,
            7 => AddDependency,
            8 => AddTaskToQueue,
            9 => AddPreSelectTask,
            10 => PreRunTask,
            11 => RunTask,
            12 => PostRunTask,
            13 => RunTaskFailed,
            14 => RemoveTask,
            15 => WaitOn,
            16 => Barrier,
            17 => AddWaitOnTask,
            id => return Err(EventError::InvalidId(id)),
        };

        Ok(event)
    }
}

/// Converts a given u8 buffer containing a c string into a rust string
/// will return an empty string if buffer contains invalid c_string
pub fn read_function_name_from_buffer(buf: &[u8]) -> String {
    // string is originally stored as CString, but we can just read in the buffer
    String::from_utf8(buf.iter().cloned().collect::<Vec<u8>>()).unwrap_or(String::new())
}

/// Converts a buffer containing u8 integers to a buffer containing u64 integers.
pub fn u8_buf_to_u64_buf(buf: &[u8]) -> Result<Vec<u64>, EventError> {
    let mut u64_buf = Vec::new();

    if buf.len() % 8 != 0 {
        return Err(EventError::BufferUnevenByteBoundary);
    }
    
    for (i, _) in buf.iter().enumerate().step_by(8) {
        let mut raw_number: [u8; 8] = [0; 8];
        raw_number.copy_from_slice(&buf[i..i + 8]);
        u64_buf.push(u64::from_be_bytes(raw_number));
    }

    Ok(u64_buf)
}

/// Gets the current timestamp
pub fn get_timestamp(buf: &[u8]) -> Option<Duration> {
    if buf.len() < 8 {
        eprintln!("Buffer needs to be at least 8 bytes long.");
        return None;
    }

    let mut new_buf = [0u8; 8];
    new_buf.clone_from_slice(&buf[buf.len() - 8..]);

    let nanos = u64::from_be_bytes(new_buf);

    Some(Duration::from_nanos(nanos))
}

#[test]
fn test_string_from_buffer() {
    let expected = String::from("Hello my name is...");
    let c_string = std::ffi::CString::new(expected.clone()).unwrap();
    let actual = read_function_name_from_buffer(c_string.as_bytes_with_nul());

    assert_eq!(actual, expected);
}

#[test]
fn test_f_name_from_string() {
    use core::ffi::CStr;
    // name = func
    let expected = String::from("func");
    let buf = [102, 117, 110, 99, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 4, 
               0, 0, 0, 0, 0, 0, 0, 4, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               23, 27, 125, 89, 31, 98, 134, 231];
    
    let actual = read_function_name_from_buffer(&buf[..4]);
    println!("{:?}", CStr::from_bytes_with_nul(&buf[..4]));
    assert_eq!(actual, expected);
}

#[test]
fn test_get_timestamp() {
    let expected = Some(
        Duration::from_nanos(
            u64::from_be_bytes([23, 27, 125, 89, 31, 98, 134, 231])
        )
    );
    let buf_long = [102, 117, 110, 99, 0, 0, 0, 0, 
                    0, 0, 0, 0, 0, 0, 0, 0, 
                    0, 0, 0, 0, 0, 0, 0, 4, 
                    0, 0, 0, 0, 0, 0, 0, 4, 
                    0, 0, 0, 0, 0, 0, 0, 0, 
                    0, 0, 0, 0, 0, 0, 0, 0, 
                    0, 0, 0, 0, 0, 0, 0, 0, 
                    23, 27, 125, 89, 31, 98, 134, 231];
    let actual_long = get_timestamp(&buf_long);
    assert!(actual_long.is_some());
    assert_eq!(actual_long, expected);

    let buf_short = [23, 27, 125, 89, 31, 98, 134, 231];
    let actual_short = get_timestamp(&buf_short);
    assert!(actual_short.is_some());
    assert_eq!(actual_short, expected);

    let buf_invalid = [23, 27, 125, 89, 31, 98, 134];
    let actual_invalid = get_timestamp(&buf_invalid);
    assert!(actual_invalid.is_none());
}

#[test]
fn test_event_type_try_from_slice() {
    let buf: [u8; 64] = [0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 4, 
               0, 0, 0, 0, 0, 0, 0, 3, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               23, 27, 188, 20, 119, 241, 215, 47];
    
    let actual = EventType::try_from(buf.as_slice());
    assert!(actual.is_ok());
    assert_eq!(actual.unwrap(), EventType::RegisterFunction);
}

#[test]
fn test_u8_buf_to_u64_buf() {
    let buf = [128, 64, 32, 255, 0, 0, 0, 1];
    let expected = vec![u64::from_be_bytes(buf.clone())];
    assert_eq!(u8_buf_to_u64_buf(buf.as_slice()).unwrap(), expected);

    let buf = [0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 4, 
               0, 0, 0, 0, 0, 0, 0, 3, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               0, 0, 0, 0, 0, 0, 0, 0, 
               23, 27, 188, 20, 119, 241, 215, 47];

    let expected = vec![0, 0, 4, 3, 0, 0, 0, u64::from_be_bytes([23, 27, 188, 20, 119, 241, 215, 47])];
    assert_eq!(u8_buf_to_u64_buf(buf.as_slice()).unwrap(), expected);

    let buf = [128, 64, 32];
    assert!(u8_buf_to_u64_buf(buf.as_slice()).is_err());
}