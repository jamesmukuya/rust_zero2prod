use std::net::TcpListener;

/*
first integration test
*/
#[tokio::test]
async fn health_check_works(){
    //1. Arrange
    let address = spawn_app();
    //`reqwest` does HTTP requests against our application
    let client = reqwest::Client::new();
    
    //2. Act
    //let response = client.get("http://127.0.0.1:8080/health_check")
    // use the random address generated
    let response = client.get(&format!("{}/health_check",&address))
        .send()
        .await
        .expect("FAILED to execute request");

    //3. Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())

}

// runs the lib.rs/run
// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
fn spawn_app() -> String{
    //static port allocation
    //let server = zero2prod::run("127.0.0.1",0).expect("failed to bind address");
    //choosing a random port to prevent health checker from failing
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port =listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);

    println!("health check {:?}",&port);
    //return address to the caller
    format!("http://127.0.0.1:{}",port)
}