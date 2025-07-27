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
	pub name: String /* respin */,
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
pub struct Respin {
	pub action: Action,
	pub autogame: bool /* false, true */,
	pub command: String /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 309 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "44f4e29b-7216-479f-8feb-c3f3b27f57b5" */,
	pub quick_spin: i64 /* 0, 2 */,
	pub request_id: String /* "72288b8c-aa1a-4f9c-8705-129403909b42" */,
	pub session_id: String /* "54d657fdddea4c76800b216371ea868e" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false, true */,
}

impl From<client::Client> for Respin {
	fn from(obj: client::Client) -> Self {
		Respin {
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

