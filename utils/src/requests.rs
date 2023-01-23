/****t* Ayudame_types.h/ayu_request_t
 *  NAME
 *    ayu_request_t - enum of requests
 *  DESCRIPTION
 *    ayu_event_t consists of the following events:
 *    * AYU_REQUEST_NULL -- "no request", used for initialisation
 *    * AYU_NOREQUEST -- explicit "no request"
 *    * AYU_PAUSEONEVENT -- pause request, event upon which the program should
 *      pause is given as third parameter, fourth parameter is 0 for "off"
 *      ("un-pause") and 1 for "on"
 *    * AYU_PAUSEONTASK --  pause request, task id is given as third parameter,
 *      fourth parameter is 0 for "off" ("un-pause") and 1 for "on"
 *    * AYU_PAUSEONFUNCTION -- pause request, function id is given as third
 *      parameter, fourth parameter is 0 for "off" ("un-pause") and 1 for "on"
 *    * AYU_STEP -- run until next pause condition is reached
 *    * AYU_BREAKPOINT -- set breakpoint, i.e. don't assign new tasks, third
 *      parameter is 0 for "off" ("un-pause") and 1 for "on"
 *    * AYU_BLOCKTSK -- to block a specific task. Task id has to be passed.
 *    * AYU_PRIORITISETASK -- to set the priority level of a specific task.
 *    * AYU_SETNUMTHREADS -- to set the number of processing resources.
 *  SOURCE
 */

use std::fmt::Display;

pub enum RequestError {
    InvalidId(i64)
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            RequestError::InvalidId(id) => format!("Invalid id for request: {}", id),
        };
        write!(f, "{}", msg)
    }
}

#[derive(Clone, Copy)]
pub enum Request {
    Null = 0,
    NoRequest = 1,
    PauseOnEvent = 2,
    PauseOnTask = 3,
    PauseOnFunction = 4,
    Step = 5,
    Breakpoint = 6,
    BlockTask = 7,
    PrioritiseTask = 8,
    SetNumThreads = 9,
    Continue = 10,
    Break = 11,
    BreakAtTask = 12,
    UnbreakAtTask = 13,
}

impl TryFrom<i64> for Request {
    type Error = RequestError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        use Request::*;
        match value {
            0 => Ok(Null),
            1 => Ok(NoRequest),
            2 => Ok(PauseOnEvent),
            3 => Ok(PauseOnTask),
            4 => Ok(PauseOnFunction),
            5 => Ok(Step),
            6 => Ok(Breakpoint),
            7 => Ok(BlockTask),
            8 => Ok(PrioritiseTask),
            9 => Ok(SetNumThreads),
            10 => Ok(Continue),
            11 => Ok(Break),
            12 => Ok(BreakAtTask),
            13 => Ok(UnbreakAtTask),
            id => Err(RequestError::InvalidId(id))
        }
    }
}