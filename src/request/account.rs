use std::fmt;
use std::fmt::Formatter;
use reqwest::{Error, Response};
use serde::{Serialize};

use crate::{
    request::{/*AuthenticatedUser,*/ REQUEST_BASE_URL}
};
use crate::request::AuthenticatedUser;

pub const ACCOUNT_LOGIN_ENPOINT: &str = "accounts/loginGJAccount.php";
pub const ACCOUNT_SECRET: &str = "Wmfv3899gc9";

#[derive(Debug, Clone, Serialize, Hash)]
pub struct LoginRequest<'a> {

    /// The Unique Device IDentifier (UDID) of the user to authenticate
    ///
    /// ## GD Internals:
    /// This field is called `udid` in the boomlings API
    /// The value of this value can be randomly generated
    /// The digits must be between 100,000 and 100,000,000
    /// This will succeed as long as these conditions are met
    pub udid: &'a str,

    /// The username of the user to authenticate
    ///
    /// ## GD Internals:
    /// This field is called `userName` in the boomlings API
    #[serde(rename = "userName")]
    pub user_name: &'a str,

    /// The unencrypted password of the user to authenticate
    ///
    /// ## GD Internals:
    /// This field is called `password` in the boomlings API
    pub password: &'a str,

    pub secret: &'a str,

    //// The base request data
    // #[serde(borrow)]
    // pub base: BaseRequest<'a>,
}

impl<'a> LoginRequest<'a> {
    const_setter!(user_name: &'a str);
    const_setter!(password: &'a str);

    pub fn default() -> Self {
        LoginRequest{
            udid: "199095",
            user_name: "",
            password: "",
            secret: ACCOUNT_SECRET
        }
    }

    pub fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, ACCOUNT_LOGIN_ENPOINT)
    }

    async fn execute(&self) -> Result<Response, Error> {
        let reqwest_client = reqwest::Client::new();
        println!("{}?{}", self.to_url(), self.to_string());
        reqwest_client
            .post(self.to_url())
            .body(self.to_string())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await
    }

    pub async fn to_authenticated_user(&self) -> Result<AuthenticatedUser,  AuthenticationError> {
        match self.execute().await {
            Ok(login_result) => {

                let response_body = login_result.text().await.unwrap();
                if response_body.eq("-1") {
                    return Err(AuthenticationError("invalid credentials".into()))
                }

                Ok(AuthenticatedUser {
                    account_id: response_body.splitn(2, ",").next().unwrap().parse::<u64>().unwrap(),
                    password_hash: base64::encode(&xor(self.password.as_bytes().to_vec(), "37526".as_bytes()))
                })

                // Box::new(AuthenticatedUser{
                //     account_id: account_id,
                //     password_hash: password
                // })
            }
            Err(login_error) => {
                Err(AuthenticationError(login_error.to_string()))
            }
        }
    }
}


fn xor(s: Vec<u8>, key: &[u8]) -> Vec<u8> {
    let mut b = key.iter().cycle();
    s.into_iter().map(|x| x ^ b.next().unwrap()).collect()
}

// fn robtop_encode_level_password(pw: String) -> [u8; 20] {
//     let mut password = [b'0'; 20];
//     password[0] = b'1';
//
//     let mut itoa_buf = [0u8; 6];
//
//     let n = itoa::write(&mut itoa_buf[..], pw).unwrap();
//
//     // ensure the password is padded with zeroes as needed
//     for i in 0..n {
//         password[7 - n + i] = itoa_buf[i];
//     }
//
//     // We need to do the xor **before** we get the base64 encoded data
//     util::cyclic_xor(&mut password[..], "37526");
//
//     password
// }

#[derive(Debug, Clone)]
pub struct AuthenticationError(String);

impl std::error::Error for AuthenticationError {}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToString for LoginRequest<'_> {
    fn to_string(&self) -> String {
        super::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::request::account::LoginRequest;

    #[tokio::test]
    async fn serialize_login_request() {
        let request = LoginRequest::default()
            .user_name("Ryder")
            .password("PASS HERE");

        println!("{:?}", request.to_authenticated_user().await.unwrap());
    }
}
