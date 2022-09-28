#[derive(Debug)]
pub struct User {
    pub pub_key: String,
    pub sig: Option<String>,
    pub ens: Option<String>,
}
