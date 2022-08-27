use derive_more::Constructor;
use serde::Serialize;

pub trait PageContext {
	fn template_path(&self) -> &str;
	fn title(&self) -> &str;
	fn parent(&self) -> &str;
}

#[derive(Debug, Serialize)]
pub struct Home {}

impl Default for Home {
	fn default() -> Self {
		Self {}
	}
}

impl PageContext for Home {
	fn template_path(&self) -> &str {
		"home"
	}

	fn title(&self) -> &str {
		"Stash Your Clipboard!"
	}

	fn parent(&self) -> &str {
		"base"
	}
}

#[derive(Debug, Constructor, Serialize)]
pub struct ViewClip {
	pub clip: crate::Clip,
}

impl PageContext for ViewClip {
	fn template_path(&self) -> &str {
		"clip"
	}

	fn title(&self) -> &str {
		"View Clip"
	}

	fn parent(&self) -> &str {
		"base"
	}
}

#[derive(Debug, Constructor, Serialize)]
pub struct PasswordRequired {
	shortcode: crate::ShortCode,
}

impl PageContext for PasswordRequired {
	fn template_path(&self) -> &str {
		"clip_need_password"
	}

	fn title(&self) -> &str {
		"Password Required"
	}

	fn parent(&self) -> &str {
		"base"
	}
}
