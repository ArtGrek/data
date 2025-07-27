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
	pub autogame: bool /* true */,
	pub command: String /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 282 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "c6a22a27-a9f4-4b6b-bc5e-7ddf8a9679f1" */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "bcba8f61-359f-4dc8-a609-a810f768260d" */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false */,
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

