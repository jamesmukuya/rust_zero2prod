use zero2prod::run;

/*
We have exported the handlers to the lib.rs module
*/
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run()?.await
}
