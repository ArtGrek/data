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
	pub autogame: bool /* false, true */,
	pub command: String /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 304 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "45193bc9-2243-48df-861e-5f32bde4db21" */,
	pub quick_spin: i64 /* 0, 2 */,
	pub request_id: String /* "3d0e9d33-2b80-420a-a170-10b82c4a4a0c" */,
	pub session_id: String /* "54d657fdddea4c76800b216371ea868e" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false, true */,
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

