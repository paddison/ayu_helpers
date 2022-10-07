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

pub enum Request{
    Null = 0,
    NoRequest = 1,
    PauseOnEvent = 2,
    PauseOnTask = 3,
    PauseOnFunction = 4,
    Step = 5,
    Breakpoint = 6,
    BlockTask = 7,
    PrioritiseTask = 8,
    SetNumThreads = 9
}