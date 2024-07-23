use serde::{Deserialize, Serialize};
use serde_json::Value;

enum RouteParseError {
  InvalidUrl(String),
}

pub fn parse_url<'url, T>(url: &str) -> Result<T, RouteParseError>
where
  T: Deserialize<'url> + Serialize,
{
  let mut parts = url.splitn(2, '/');
  let part1 = parts.next().unwrap();
  let part2 = parts.next().unwrap_or("");
  (part1, part2)
}

pub fn stringify_url<'url, T>(route: T) -> T
where
  T: Serialize + Deserialize<'url>,
{
  format!("{}/{}", part1, part2)
}
