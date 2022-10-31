use dash_rs::request;

#[tokio::test]
async fn login_gj_account_test() {
    let request = request::account::LoginRequest::default()
        .user_name("Ryder")
        .password("PASS_HERE");

    let result = request.to_authenticated_user().await.unwrap();

    println!("{:?}", result);
}