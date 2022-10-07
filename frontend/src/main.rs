// attempt to connect to ayudame socket
// send requests via socket
// should be able to process received events

pub mod request_handlers;
pub mod ayu_event_handlers;

use std::{net::{TcpStream, SocketAddrV4, Ipv4Addr}, env::VarError, time::Duration, sync::{Arc, RwLock}, io::{Read, Write, IoSliceMut}};

use utils::AppState;

use crate::{request_handlers as requests, ayu_event_handlers::EventResult};
use crate::ayu_event_handlers as events;

const AYU_PORT: u16 = 5555;
const BUF_SIZE: usize = 8 * 8;

// Spawn to threads, one to send out requests, the other to receive requests

fn main() -> Result<(), String>{
    // tries to connect to a socket, should be read from AYU_PORT env
    let port = std::env::var("AYU_PORT")
                .and_then(|p| p.parse::<u16>()
                .map_err(|_| VarError::NotPresent))
                .unwrap_or(AYU_PORT);
    
    let event_receive_stream = TcpStream::connect(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port)).map_err(|e| format!("Unable to connect to socket: {}, aborting...", e))?; 
    let request_stream = event_receive_stream.try_clone().unwrap();

    let event_receive_state = Arc::new(RwLock::new(AppState::new()));
    let request_state = Arc::clone(&event_receive_state);

    let event_receiver = std::thread::spawn(event_receiver_loop(event_receive_state, event_receive_stream));
    let request_sender = std::thread::spawn(request_sender_loop(request_state, request_stream));

    println!("Connected to socket. Waiting for threads to finish...");
    let _ = event_receiver.join();
    let _ = request_sender.join();
    
    Ok(())
}

fn request_sender_loop(state: Arc<RwLock<AppState>>, mut stream: TcpStream) -> impl FnOnce() -> () {
    move || {
        let mut buf = requests::prepare_step(5);
        println!("Started AyuRequest Sender thread");
        // println!("State is {}", state.read().unwrap());
        loop {
            requests::print_options();
            println!("Wrote: {:?}", stream.write(&mut buf));
            std::thread::sleep(Duration::from_secs(5));
            break;
        }
    }
}

fn event_receiver_loop(mut state: Arc<RwLock<AppState>>, mut stream: TcpStream) -> impl FnOnce() -> () {
    move || {
        let mut buf = [0u8; 64];
        println!("Started AyuEvent Receiver thread");
        // println!("State is {}", state.write().unwrap());
        loop {
            let n = match stream.read(&mut buf){
                Ok(n) => n,
                Err(_) => 0,
            };
            
            if n > 0 {
                println!("Read: {} bytes", n);
                println!("{:?}", buf);
                // println!("{:?}", string_buf);
                match events::handle_event(&buf, &mut state, &mut stream) {
                    Ok(result) => if let EventResult::Exit = result { break }
                    Err(e) => eprintln!("Unable to handle received event: {}", e),
                }
            }
        }
    }
}


#[test]
fn test_to_buffer() {
    use utils::requests::Request;
    let buf = [0_usize.to_be_bytes(), (Request::Step as usize).to_be_bytes(), 1_usize.to_be_bytes()].into_iter().flatten().collect::<Vec<u8>>();

    println!("{:?}", buf);
}