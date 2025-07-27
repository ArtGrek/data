use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{ActionNameEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 1 */,
}

impl From<client::Params> for Params {
	fn from(obj: client::Params) -> Self {
		Params {
			bet_per_line: obj.bet_per_line,
			lines: obj.lines,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionNameEnum /* bonus_spins_stop, respin, spin */,
	pub params: Params,
}

impl From<client::Action> for Action {
	fn from(obj: client::Action) -> Self {
		Action {
			name: obj.name.into(),
			params: obj.params.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Unknown {
	pub action: Action,
	pub autogame: bool /* true */,
	pub command: String /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 72440 */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "852a733c-fe92-4958-babd-2b80f9481ec4" */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false */,
}

impl From<client::Client> for Unknown {
	fn from(obj: client::Client) -> Self {
		Unknown {
			action: obj.action.into(),
			autogame: obj.autogame,
			command: obj.command,
			mobile: obj.mobile,
			portrait: obj.portrait,
			prev_client_command_time: obj.prev_client_command_time,
			quick_spin: obj.quick_spin,
			request_id: obj.request_id,
			session_id: obj.session_id,
			set_denominator: obj.set_denominator,
			sound: obj.sound,
		}
	}
}

