use std::io::prelude::*;
use std::net::TcpStream;
use kaf::read_all;
use kaf::update_list;
use kaf::return_update;

pub fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let post = b"POST / HTTP/1.1\r\n";
    let delete=b"DELETE / HTTP/1.1\r\n";
    let  status_line:String;
    let contents:String;
    
    if buffer.starts_with(get){

        status_line=String::from("HTTP/1.1 200 OK");
        let v=read_all();
        contents=v;
    }
    else 
    if buffer.starts_with(post)
    {
        status_line=String::from("HTTP/1.1 200 OK");
        contents=String::from("Post Request"); 
        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line=request.lines().last().expect("Unable to read the post request").trim_end_matches(char::from(0)).to_string();
        println!("{}",request_line);

        update_list(request_line);

    } else if buffer.starts_with(delete) {
        status_line=String::from("HTTP/1.1 200 OK");
        //contents=String::from("Delete request");
        contents=return_update();
    } else 
    {
        status_line=String::from("HTTP/1.1 404 NOT FOUND");
        contents=String::from("Not found !!");
    }

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}