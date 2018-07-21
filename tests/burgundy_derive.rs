#[macro_use]
extern crate burgundy;
use burgundy::Request;

#[test]
fn test_building_url() {
  #[derive(Request)]
  #[allow(dead_code)]
  #[request(method="get", path="/repo/{}/{}")]
  struct GithubRepos {
    owner : String
  }

  let repos = GithubRepos {
    owner : "Microsoft".to_string(),
  };

  assert_eq!(repos.to_url_path(), "/repos/Microsoft");
}