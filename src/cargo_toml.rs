use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct  Author {
    pub name: String,
    pub organization: String,
    pub email: String,
}

