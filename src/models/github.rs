use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Organization {
    pub login: String,
    pub url: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub website_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub items: Vec<Organization>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub login: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrgResult {
    pub login: String,
    pub website_url: Option<String>,
    pub email: Option<String>,
    pub domain_verified: bool,
    pub shared_members: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub stargazers_count: u32,
    pub language: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub releases: Vec<Release>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub name: String,
}
