use std::collections::HashMap;
use std::fmt::LowerHex;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::ptr::read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::{io::*, thread};
use rand::Rng;

pub fn put_name(gt_pt_dlt_obj_name: &Mutex<HashMap<String, String>>, new_id: String, nm: String){
    gt_pt_dlt_obj_name.lock().unwrap().insert(new_id, nm);
}


fn dlt_name(gt_pt_dlt_obj_name: &Mutex<HashMap<String, String>>, id: &String){
    gt_pt_dlt_obj_name.lock().unwrap().remove(id);
}


fn get_name(gt_pt_dlt_obj_name: &Mutex<HashMap<String, String>>, id: String){
    //gt_pt_dlt_obj_name.lock().unwrap().remove(id);
    gt_pt_dlt_obj_name.lock().unwrap().entry(id);
    let k = 2.to_string();
    let binding = gt_pt_dlt_obj_name.lock().unwrap();
    let data = binding.get(&k);
    println!("{:?}", data);
}


fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request/*: Vec<_>*/ = buf_reader.lines().next().unwrap().unwrap();
    
    let rquest: Vec<_> = http_request.split(" ").collect();
    let rq_tp = rquest[0].to_string();
    
    
    //print!("{:#?}",rq_tp); print!("{:#?}",id); print!("{:#?}",tp); print!("{:#?}",nm);
    
    let obj_name: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    obj_name.lock().unwrap().insert(1.to_string(),"H1312_0024_10010269_T".to_string());
    obj_name.lock().unwrap().insert(2.to_string(),"DS_H1312_0024_WORKCNTR".to_string());
    obj_name.lock().unwrap().insert(3.to_string(),"FU_H1312_0024_10010269_P".to_string());

    
    let put_obj_name = obj_name.clone();
    let dlt_obj_name = obj_name.clone();
    let get_obj_name = obj_name.clone();
    
   
    let obj_type: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    obj_type.lock().unwrap().insert(1.to_string(),"AP".to_string());
    obj_type.lock().unwrap().insert(2.to_string(),"DS".to_string());
    obj_type.lock().unwrap().insert(3.to_string(),"FU".to_string());
    
    let put_obj_type = obj_type.clone();

    match rq_tp.as_str() {
        "get" => {
            let id = rquest[1].to_string();
            println!("{:#?}",id); 
            println!("Receive get from client");
        
            get_name(&get_obj_name, id);
            // echo everything!
            //let response = "Sent get to client";
            //stream.write(response.as_bytes()).unwrap();
        },
        "put" => {           
            let new_id = rand::thread_rng().gen_range(1..=100).to_string();
            let new_id_nm = new_id.to_string();
            let new_id_tp = new_id.to_string();
            
            let nm = rquest[1].to_string();
            let tp = rquest[2].to_string();
            
            put_obj_name.lock().unwrap().insert(new_id_nm, nm);
            put_obj_type.lock().unwrap().insert(new_id_tp, tp);
            //put_name(&put_obj_name, new_id, nm);
            //thread::spawn(move || new_name(&add_obj_name, tp));
            
        },
        "delete" => {
            let id = rquest[1].to_string(); 
            println!("Receive delete");
            //thread::spawn(move || dlt_name(&dlt_obj_name, &id));
            dlt_name(&dlt_obj_name, &id);
        },
        /*"gen"=>{
            let inj_code1:[&str; 16] = ["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F"];
            let inj_code2 = ["0","1","2","3","4","5","6","7","8","9","A","B","C","D","E","F"];
            let mut new_code:String;
            for char1 in 0..16 {
                for char2 in 0..16{
                    new_code = inj_code1[char1].to_string() + &inj_code2[char2].to_string();
                    println!("{}", new_code)
                };
            };

        }, */
        _ => println!("Receive X3"),
    }
    /*/*.map(|result| result.unwrap())
    .take_while(|line|!line.is_empty())
    .collect(); */
    if http_request == "Hello"/*"GET / HTTP/1.1"*/{
        println!("Request: {:?}", http_request);
        let response = "Hello"/*"HTTP/1.1 200 OK\r\n\r\n" */;
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        println!("Request: {:?}", http_request);
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }; */
   /* let mut data = [0 as u8; 50]; // using 50 byte buffer
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
    } */ {}


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