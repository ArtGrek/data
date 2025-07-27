use super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::server::{SyncModesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: String /* OK */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 507200 */,
	pub balance_version: i64 /* 1752068250132 */,
	pub currency: String /* FUN */,
	pub huid: String /* "686e6f522c7c80483b132b54" */,
	pub nick: String /* Player 3a9b80b6-e351-4713-9627-3ad37a961139 */,
	pub show_balance: bool /* true */,
}

impl From<server::User> for User {
	fn from(obj: server::User) -> Self {
		User {
			balance: obj.balance,
			balance_version: obj.balance_version,
			currency: obj.currency,
			huid: obj.huid,
			nick: obj.nick,
			show_balance: obj.show_balance,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Sync {
	pub command: String /* sync */,
	pub modes: Vec<SyncModesEnum> /* auto, play */,
	pub request_id: String /* "92cb36c58d1640eca71d6025d16dc37b" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for Sync {
	fn from(obj: server::Server) -> Self {
		Sync {
			command: obj.command,
			modes: obj.modes.into_iter().map(Into::into).collect(),
			request_id: obj.request_id,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.into(),
		}
	}
}

