use crate::domain::clip::field;
use crate::ShortCode;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClip {
	pub shortcode: ShortCode,
	pub password: field::Password,
}

impl GetClip {
	pub fn from_raw(shortcode: &str) -> Self {
		Self {
			shortcode: ShortCode::from(shortcode),
			password: field::Password::default(),
		}
	}
}

impl From<&str> for GetClip {
	fn from(raw: &str) -> Self {
		Self::from_raw(raw)
	}
}

impl From<ShortCode> for GetClip {
	fn from(shortcode: ShortCode) -> Self {
		Self {
			shortcode,
			password: field::Password::default(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewClip {
	pub content: field::Content,
	pub title: field::Title,
	pub expires: field::Expires,
	pub password: field::Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateClip {
	pub shortcode: field::ShortCode,
	pub content: field::Content,
	pub title: field::Title,
	pub expires: field::Expires,
	pub password: field::Password,
}
