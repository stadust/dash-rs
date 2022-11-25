use dash_rs::request;

#[tokio::test]
async fn login_gj_account_test() {
    dotenv::from_filename("test_env.env").expect("test_env.env file not found");

    let user_name = dotenv::var("GJ_ACCOUNT_USERNAME").unwrap();
    let password = dotenv::var("GJ_ACCOUNT_PASSWORD").unwrap();

    let request = request::account::LoginRequest::default()
        .user_name(&user_name)
        .password(&password);

    let result = request.to_authenticated_user().await.unwrap();
    println!("{:?}", result);
}