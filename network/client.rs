use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;


fn main() -> std::io::Result<()> {
    let  stream = TcpStream::connect("127.0.0.1:8082")?;

    // stream.write(&[1])?;
    // stream.read(&mut [0; 128])?;
    // 一回読むと消えるので、ここで読むと下の read_line 時の print がされない

    println!("{:?}", stream);


        let mut recv_response = String::new();
        let mut reader = BufReader::new(stream);

        if let Ok(v) = reader.read_line(&mut recv_response) {
            if v > 0 {
                //レスポンスを表示
                println!("response: {}",recv_response);
            }
        }



    Ok(())
} // the stream is closed here