use dash_rs::request;
use dash_rs::request::{BaseRequest};

macro_rules! blocking_await {
  ($e:expr) => {
      tokio_test::block_on($e)
  };
}

#[tokio::test]
async fn login_gj_account_test() {
    let base = BaseRequest::default();
    let test = request::account::LoginRequest::with_base(base)
        .user_name("Ryder")
        .password("PASS HERE")
        .execute()
        .await.unwrap();

    println!("{:?}", test)
}