use derive_more::Constructor;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Constructor, Serialize, Deserialize)]
pub struct Hits(u64);

impl Hits {
	pub fn into_inner(self) -> u64 {
		self.0
	}
}
