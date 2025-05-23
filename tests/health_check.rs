//use zero2prod::main;

/*
first integration test
*/
#[tokio::test]
async fn health_check_works(){
    //1. Arrange
    spawn_app();
    //`reqwest` does HTTP requests against our application
    let client = reqwest::Client::new();
    
    //2. Act
    let response = client.get("http://127.0.0.1:8080/health_check")
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
fn spawn_app(){
    //todo!()
    let server = zero2prod::run().expect("failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}