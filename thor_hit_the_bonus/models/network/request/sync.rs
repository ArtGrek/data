use super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::client::{};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Sync {
	pub command: String /* sync */,
	pub request_id: String /* "92cb36c58d1640eca71d6025d16dc37b" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
}

impl From<client::Client> for Sync {
	fn from(obj: client::Client) -> Self {
		Sync {
			command: obj.command,
			request_id: obj.request_id,
			session_id: obj.session_id,
		}
	}
}

