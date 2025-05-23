use zero2prod::run;

/*
We have exported the handlers to the lib.rs module
*/
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run().await
}
