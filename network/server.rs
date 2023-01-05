use std::net::{TcpListener, TcpStream};
use std::io::{Write};


fn _handle_client(stream: TcpStream) -> std::io::Result<()> {
    // ...
    // let s = stream.try_clone()
    // let mut reader = BufReader::new(s);

    println!("---------");
    println!("{:?}", stream);
    // println!("{:?}", reader);
    println!("---------");
    // stream.write(&[1])?;
    // stream.read(&mut [0; 128])?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8082")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        // handle_client(stream?);
        println!("---------");
        println!("{:?}", stream);

        // stream?.write(&[1])?;
        let mut socket = stream?.try_clone().unwrap();


        let response = String::from(format!("client recv {}","helo")).into_bytes();
        match socket.write(&response) {
            Ok(_) => println!("client response success"),
            Err(v) => println!("client response failed:{}",v),
        }
    }
    Ok(())
}