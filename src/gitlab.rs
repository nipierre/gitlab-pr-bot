use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GitLab {
    pub url: String,
    pub token: String
}
