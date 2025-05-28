use std::net::TcpListener;

use zero2prod::run;

/*
We have exported the handlers to the lib.rs module
*/
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let main_address = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("{:?}",&main_address);
    run(main_address)?.await
}
