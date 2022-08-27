use crate::domain::time::Time;
use derive_more::Constructor;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Constructor, Serialize, Deserialize)]
pub struct Posted(Time);

impl Posted {
	pub fn into_inner(self) -> Time {
		self.0
	}
}