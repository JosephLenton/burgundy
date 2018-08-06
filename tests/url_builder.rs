#![allow(dead_code)]

#[macro_use]
extern crate burgundy;
use burgundy::Request;

#[test]
fn test_building_url() {
  #[derive(Request)]
  #[request(method="Get", path="/repo/{}/{}")]
  struct GithubRepos {
    owner : String,

    #[query]
    page : u32,
  }

  let repos = GithubRepos {
    owner : "Microsoft".to_string(),
    page : 5,
  };

  assert_eq!(repos.to_url_path(), "/repos/Microsoft");
}