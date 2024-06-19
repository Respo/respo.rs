use serde_json::Value;

/// component level state that could be backuped
pub trait RespoState {
  fn backup(&self) -> Option<Value> {
    None
  }
  fn restore_from(&mut self, _s: &Value) -> Result<(), String> {
    Ok(())
  }
}

impl RespoState for bool {
  fn backup(&self) -> Option<Value> {
    Some(Value::Bool(*self))
  }

  fn restore_from(&mut self, s: &Value) -> Result<(), String> {
    *self = s.as_bool().ok_or_else(|| "Expected a boolean value".to_string())?;
    Ok(())
  }
}

impl RespoState for () {
  fn backup(&self) -> Option<Value> {
    None
  }

  fn restore_from(&mut self, _s: &Value) -> Result<(), String> {
    Ok(())
  }
}
