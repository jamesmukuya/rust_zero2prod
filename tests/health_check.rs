use std::{net::TcpListener, vec};

//use actix_web::test;

/*
integration tests
*/

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
    assert_eq!(Some(19), response.content_length())

}

/*
Test case when user submits valid name and email address
*/
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data(){
    //1. Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    //2. Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client.post(&format!("{}/subscriptions",&app_address))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //3. Assert
    assert_eq!(200, response.status().as_u16());
}

/*
Test if user has submits invalid or missing data into the form
*/
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing(){
    //1. Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    //Create test case for username, email or both
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases{
        //2. Act
        let response = client.post(&format!("{}/subscriptions",&app_address))
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");
        
        //3. Assert
        assert_eq!(400, response.status().as_u16(),
        "The API did not fail with 400 Bad Request when the payload was {}.",error_message);
    }

}