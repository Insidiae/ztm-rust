use crate::domain::clip::ClipError;
use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Password(Option<String>);

impl Password {
	pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
		let password: Option<String> = password.into();

		match password {
			Some(password) => {
				if !password.trim().is_empty() {
					Ok(Self(Some(password)))
				} else {
					Ok(Self(None))
				}
			}
			None => Ok(Self(None))
		}
	}

	pub fn into_inner(self) -> Option<String> {
		self.0
	}

	pub fn has_password(&self) -> bool {
		self.0.is_some()
	}
}

impl Default for Password {
	fn default() -> Self {
		Self(None)
	}
}

impl FromStr for Password {
	type Err = ClipError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::new(String::from(s))
	}
}