#[derive(Debug)]
pub struct Repo {
    pub remote: bool,
    pub provider: String,
    pub user: String,
    pub repo: String,
    pub description: Option<String>,
}

impl Repo {
    pub fn from(provider: &str, user: &str, repo: &str) -> Self {
        Self {
            remote: true,
            provider: String::from(provider),
            user: String::from(user),
            repo: String::from(repo),
            description: None,
        }
    }

    pub fn local() -> Self {
        Self {
            remote: false,
            provider: "".to_string(),
            user: "".to_string(),
            repo: "".to_string(),
            description: None,
        }
    }

    pub fn with_descripion(description: String) -> Self {
        let mut local = Self::local();
        local.description = Some(description);
        local
    }
}
