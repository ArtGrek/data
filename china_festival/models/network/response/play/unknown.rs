use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::server::{UnknownModesEnum, StatusCodeEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: StatusCodeEnum /* GAME_REOPENED, PLAYER_DISCONNECTED, SERVER_ERROR, SESSION_LOST */,
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
	pub balance: i64 /* 9850 */,
	pub balance_version: i64 /* 82 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-4a199296f4c94668aa7645a1f9bcf0f1" */,
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
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command: Option<String> /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modes: Option<Vec<UnknownModesEnum>> /* auto, play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub request_id: Option<String> /* "415df5f8-1c06-4e37-aa53-6f437c7fe8b8" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String> /* "55ea9ad42dac4b82a528beea90e1a81d" */,
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

