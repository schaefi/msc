#[derive(Debug)]
pub struct Obs {
    pub api_server: String,
    pub user: String,
    pub password: String,
    pub project: String
}

impl Obs {
    pub fn new(
        api_server: String,
        user: String,
        password: String,
        project: String
    ) -> Obs {
        Obs{
            api_server: api_server, user: user,
            password: password, project: project
        }
    }
}
