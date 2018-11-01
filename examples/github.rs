//! This is an incomplete example of bilding a Github bindings using
//! Burgundy.

extern crate burgundy;

pub struct Github {
    domain: burgundy::Domain,
}

impl Github {
    pub fn new() -> Self {
        let mut domain = burgundy::Domain::new("https://api.github.com");
        domain.header(
            &"User-Agent",
            &format!("github burgundy example / {}", env!("CARGO_PKG_VERSION")),
        );

        Self {
            domain,
        }
    }

    pub fn get(&self) -> GithubGet {
        GithubGet {
            url: self.domain.get(),
        }
    }
}

pub struct GithubGet {
    url: burgundy::Path,
}

impl GithubGet {
    pub fn orgs(
        self,
        org: &str,
    ) -> GithubGetOrgs {
        GithubGetOrgs {
            url: self.url.push(&"orgs").push(&org),
        }
    }
}

pub struct GithubGetOrgs {
    url: burgundy::Path,
}

impl GithubGetOrgs {
    pub fn repos(self) -> GithubGetOrgsRepos {
        GithubGetOrgsRepos {
            url: self.url.push(&"repos"),
        }
    }
}

pub struct GithubGetOrgsRepos {
    url: burgundy::Path,
}

impl GithubGetOrgsRepos {
    pub fn run(self) -> Result<String, burgundy::Error> {
        self.url.execute_as_string::<()>(None)
    }
}

fn main() -> Result<(), burgundy::Error> {
    let github = Github::new();
    let path = github.get().orgs("Microsoft").repos();

    let repos = path.run()?;
    println!("{}", repos);

    Ok(())
}
