use utils::requests::Request;

// request types are at index 1 in buffer
// all values are stored as int
// buffer is 8 in size_t

// never used
pub fn prepare_null() {}

// doesn't store any data
pub fn prepare_no_request() {}

// event is at 2, value at 3, either 0 or 1
pub fn prepare_pause_on_event(is_on: isize) {}

// value is at 3, task_id at 2
pub fn prepare_pause_on_task(task_id: isize, is_on: isize) {}

// is not handled at all
pub fn prepare_pause_on_function() {}

// value is at 2, 
pub fn prepare_step(step: i64) -> Vec<u8> {
    let buf = [0_usize.to_be_bytes(), (Request::Step as usize).to_be_bytes(), 
               step.to_be_bytes(), 0_usize.to_be_bytes(),
               0_usize.to_be_bytes(), 0_usize.to_be_bytes(),
               0_usize.to_be_bytes(), 0_usize.to_be_bytes()
               ].into_iter()
                .flatten()
                .collect::<Vec<u8>>();

    println!("{:?}", buf);
    buf
}

// value either 0 or 1, at index 3
pub fn prepare_breakpoint(is_on: isize) {}

// id at 2 value at 3, value 1 indicates insert
pub fn prepare_block_task(task_id: isize, is_blocked: isize) {}

// id at 2, value at 3, value is priority level
pub fn prepare_prioritise_task(task_id: isize, priority: isize) {}

// value at 2, gets checked with max_threads variable which is set in init
// calls cpp function (needs to be included in rust as ffi function)
pub fn prepare_set_num_threads(n_threads: isize) {}

pub fn get_request_type() {
    
}

pub fn print_options() {

}