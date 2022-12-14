use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod field;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
	#[serde(skip)]
	pub clip_id: field::ClipId,
	pub shortcode: field::ShortCode,
	pub content: field::Content,
	pub title: field::Title,
	pub posted: field::Posted,
	pub expires: field::Expires,
	pub password: field::Password,
	pub hits: field::Hits,
}

#[derive(Debug, Error)]
pub enum ClipError {
	#[error("Invalid Password: {0}")]
	InvalidPassword(String),
	#[error("Invalid Title: {0}")]
	InvalidTitle(String),
	#[error("Empty Content")]
	EmptyContent,
	#[error("Invalid Date: {0}")]
	InvalidDate(String),
	#[error("Date Parse Error: {0}")]
	DateParse(#[from] chrono::ParseError),
	#[error("ID Parse Error: {0}")]
	Id(#[from] uuid::Error),
	#[error("Hits Parse Error: {0}")]
	Hits(#[from] std::num::TryFromIntError),
}
