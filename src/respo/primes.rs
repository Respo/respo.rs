use std::collections::HashMap;

pub enum RespoElement {
  Component(String, Vec<Box<dyn FnMut() -> Result<(), String>>>, Box<RespoElement>),
  Element(RespoElementProps, Vec<RespoElement>),
}

pub struct RespoElementProps {
  pub attrs: HashMap<String, String>,
  pub event: HashMap<String, Box<dyn Fn() -> Result<(), String>>>,
  pub styles: HashMap<String, String>,
}

pub struct RespoEvent {
  name: String,
  coord: Vec<u32>,
}
