use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Namespace {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub kind: String,
    pub full_path: String,
    pub parent_id: Option<u32>,
    pub avatar_url: Option<String>,
    pub web_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Repo {
    pub id: u32,
    pub description: String,
    pub name: String,
    pub name_with_namespace: String,
    pub path: String,
    pub path_with_namespace: String,
    pub created_at: String,
    pub default_branch: String,
    pub tag_list: Vec<String>,
    pub ssh_url_to_repo: String,
    pub http_url_to_repo: String,
    pub web_url: String,
    pub readme_url: Option<String>,
    pub avatar_url: Option<String>,
    pub forks_count: usize,
    pub star_count: usize,
    pub last_activity_at: String,
    pub namespace: Namespace,
}
