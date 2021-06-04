pub mod api; // 3.
pub mod handlers; // 2.
pub mod routes; // 1.
pub mod view;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginFormBody {
    pub login_challenge: Option<String>,
    pub username: String,
    pub password: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginQueryParams {
    pub login_challenge: Option<String>,
}
