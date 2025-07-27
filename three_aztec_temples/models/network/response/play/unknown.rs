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
	pub balance: i64 /* 89300 */,
	pub balance_version: i64 /* 2665 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-34eab375d13b45eabe01e73b5fa68e22" */,
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
	pub request_id: String /* "b04a6a2f-275a-40eb-8398-459b3b1851bf" */,
	pub session_id: String /* "e469cd358c7b434b8194a6901e3c78ca" */,
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

