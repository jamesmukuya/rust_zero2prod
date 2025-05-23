//use zero2prod::main;

/*
first integration test
*/
#[tokio::test]
async fn health_check_works(){
    //1. Arrange
    spawn_app().await.expect("Unable to launch app");
    //`reqwest` does HTTP requests against our application
    let client = reqwest::Client::new();
    
    //2. Act
    let response = client.get("127.0.0.1/health_check")
        .send()
        .await
        .expect("FAILED to execute request");

    //3. Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())

}

// runs the lib.rs/run
async fn spawn_app() -> Result<(), std::io::Error>{
    //todo!()
    zero2prod::run().await
}