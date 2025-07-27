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
	pub balance: i64 /* 97765 */,
	pub balance_version: i64 /* 121 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-106758a99a3346fba872f844aa187a8c" */,
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
pub struct Sync {
	pub command: String /* sync */,
	pub modes: Vec<SyncModesEnum> /* auto, freebet, play */,
	pub request_id: String /* "41d4764e-070d-41ce-bcc3-489b55afad15" */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
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

