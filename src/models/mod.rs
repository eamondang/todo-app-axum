use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Item {
  pub id: u64,
  pub title: String,
  pub status: bool,
}
