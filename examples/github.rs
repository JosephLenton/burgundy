#![feature(extern_in_paths)]
use extern::burgundy;

pub struct Github {
    domain: burgundy::Domain,
}

impl Github {
    pub fn new() -> Self {
        Self {
            domain: burgundy::Domain::new("https://api.github.com"),
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

pub struct GithubGetRepo {
    url: burgundy::Path,
}

impl GithubGet {
    pub fn repo(self, org: &str) -> GithubGetRepo {
        GithubGetRepo {
            url: self.url.push(&"repo").push(&org),
        }
    }
}

fn main() -> () {
    let github = Github::new();
    let url = github.get().repo("Microsoft");

    println!("{}", url.url);

    ()
}
