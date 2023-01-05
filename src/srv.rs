use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::ptr::read;
use std::sync::{Arc, Mutex};
use std::{io::*, thread, prelude::*};

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request/*: Vec<_>*/ = buf_reader.lines().next().unwrap().unwrap();
    /*.map(|result| result.unwrap())
    .take_while(|line|!line.is_empty())
    .collect(); */
    if http_request == "GET / HTTP/1.1"{
        println!("Request: {:?}", http_request);
        new_name::k = 3.to_string();
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        println!("Request: {:?}", http_request);
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    };
    /*let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}*/


    let obj_name: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let add_obj_name = obj_name.clone();
    let dlt_obj_name = obj_name.clone();

    thread::spawn(move || new_name(&add_obj_name));
    fn new_name(new_name: &Mutex<HashMap<String, String>>){
        new_name.lock().unwrap().insert(1.to_string(),"H1312_0024_10010269_T".to_string());
        new_name.lock().unwrap().insert(2.to_string(),"DS_H1312_0024_WORKCNTR".to_string());
        new_name.lock().unwrap().insert(3.to_string(),"FU_H1312_0024_10010269_P".to_string());

        let k = 2.to_string();
        let binding = new_name.lock().unwrap();
        let data = binding.get(&k);
        println!("{:?}", data);
    }

    thread::spawn(move || dlt_name(&dlt_obj_name));
    fn dlt_name(dlt_name: &Mutex<HashMap<String, String>>){
        let a_key = 1.to_string();
        dlt_name.lock().unwrap().remove(&a_key);
    }
   
    let obj_type: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let add_obj_type = obj_type.clone();

    thread::spawn(move || new_type(&add_obj_type));
    fn new_type(new_type: &Mutex<HashMap<String, String>>){
        new_type.lock().unwrap().insert(1.to_string(),"AP".to_string());
        new_type.lock().unwrap().insert(2.to_string(),"DS".to_string());
        new_type.lock().unwrap().insert(3.to_string(),"FU".to_string());
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 7878");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| handle_client(stream));
                    // connection succeeded
            }
            Err(e) => {
                println!("Error: {}", e);
                // connection failed
            }
        }
    }
    // close the socket server
    drop(listener);
    /*let mut obj_type = HashMap::new();
    obj_type.insert(1, "AP");
    obj_type.insert(2, "DS");
    obj_type.insert(3, "FU");
    */
    /*
    let obj_id = 1;
    let data = obj_name.get(&obj_id);
    let data1 = obj_type.get(&obj_id);
    println!("{:?} {:?}", new_obj::data, data1)
    */
}