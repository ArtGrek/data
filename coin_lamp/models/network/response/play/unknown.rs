use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::server::{UnknownModesEnum, StatusCodeEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodeEnum /* GAME_REOPENED, PLAYER_DISCONNECTED */,
	#[serde(rename = "type")]
	pub status_type: String /* crit */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code.into(),
			status_type: obj.status_type,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 85800 */,
	pub balance_version: i64 /* 2665 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-e55b3f9a0a5f4e42ac8114faaf0d413f" */,
	pub show_balance: bool /* true */,
}

impl From<server::User> for User {
	fn from(obj: server::User) -> Self {
		User {
			balance: obj.balance,
			balance_version: obj.balance_version,
			currency: obj.currency,
			huid: obj.huid,
			show_balance: obj.show_balance,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Unknown {
	pub command: String /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modes: Option<Vec<UnknownModesEnum>> /* auto, play */,
	pub request_id: String /* "852a733c-fe92-4958-babd-2b80f9481ec4" */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
	pub status: Status,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<User>,
}

impl From<server::Server> for Unknown {
	fn from(obj: server::Server) -> Self {
		Unknown {
			command: obj.command,
			modes: obj.modes.map(|vec| vec.into_iter().map(Into::into).collect()),
			request_id: obj.request_id,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.map(Into::into),
		}
	}
}

