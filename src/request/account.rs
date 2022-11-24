use std::fmt;
use std::fmt::Formatter;
use serde::{Serialize};

use crate::{
    util,
    request::{REQUEST_BASE_URL}
};
use crate::request::{AuthenticatedUser, Executable};

pub const ACCOUNT_LOGIN_ENPOINT: &str = "accounts/loginGJAccount.php";
pub const XOR_KEY: &str = "37526";

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

    /// The secret token to call /database/accounts routes
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
            udid: "100000",
            user_name: "",
            password: "",
            secret: super::ACCOUNT_SECRET
        }
    }

    pub async fn to_authenticated_user(&self) -> Result<AuthenticatedUser,  AuthenticationError> {
        match self.execute().await {
            Ok(login_result) => {
                let response_body = login_result.text().await.unwrap();
                if response_body.eq("-1") {
                    return Err(AuthenticationError("invalid credentials".into()))
                }

                Ok(AuthenticatedUser {
                    user_name: self.user_name,
                    account_id: response_body.splitn(2, ",").next().unwrap().parse::<u64>().unwrap(),
                    password_hash: base64::encode_config(&util::xor(self.password.as_bytes().to_vec(), XOR_KEY.as_bytes()), base64::URL_SAFE).into()
                })
            }
            Err(login_error) => {
                Err(AuthenticationError(login_error.to_string()))
            }
        }
    }
}

impl<'a> Executable for LoginRequest<'a> {
    fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, ACCOUNT_LOGIN_ENPOINT)
    }
}

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
            .user_name("TestUser")
            .password("PLAIN_TEXT_PASS_HERE");

        assert_eq!(
            request.to_string(),
            "udid=100000&userName=TestUser&password=PLAIN_TEXT_PASS_HERE&secret=Wmfv3899gc9"
        );
    }
}
