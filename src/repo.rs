pub struct Repo {
    pub owner: String,
    pub repo: String,
}

impl Repo {
    pub fn parse(project: String) -> Self {
        let components: Vec<&str> = project.split("/").collect();
        Self {
            owner: components[0].to_owned(),
            repo: components[1].to_owned(),
        }
    }
}
