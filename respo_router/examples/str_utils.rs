use serde::{Deserialize, Serialize};

extern crate respo_router;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Demo {
  Home,
  App(u16),
  User(String),
  NotFound,
}

fn main() {
  println!("{:?}", respo_router::parse_url::<Demo>("hello/world"))
}
