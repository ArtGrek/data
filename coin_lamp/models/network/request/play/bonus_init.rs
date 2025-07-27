use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
}

impl From<client::Params> for Params {
	fn from(obj: client::Params) -> Self {
		Params {
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: String /* bonus_init */,
	pub params: Params,
}

impl From<client::Action> for Action {
	fn from(obj: client::Action) -> Self {
		Action {
			name: obj.name,
			params: obj.params.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BonusInit {
	pub action: Action,
	pub autogame: bool /* true */,
	pub command: String /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 72440 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "01c6c84b-2b58-4d3a-85ca-a14faa0411c6" */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "5513d70b-f0b0-41e8-bc03-907809dfa116" */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false */,
}

impl From<client::Client> for BonusInit {
	fn from(obj: client::Client) -> Self {
		BonusInit {
			action: obj.action.into(),
			autogame: obj.autogame,
			command: obj.command,
			mobile: obj.mobile,
			portrait: obj.portrait,
			prev_client_command_time: obj.prev_client_command_time,
			prev_request_id: obj.prev_request_id,
			quick_spin: obj.quick_spin,
			request_id: obj.request_id,
			session_id: obj.session_id,
			set_denominator: obj.set_denominator,
			sound: obj.sound,
		}
	}
}

