use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{ActionNameEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 1, 150 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 25 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<String> /* 2 */,
}

impl From<client::Params> for Params {
	fn from(obj: client::Params) -> Self {
		Params {
			bet_factor: obj.bet_factor,
			bet_per_line: obj.bet_per_line,
			lines: obj.lines,
			selected_mode: obj.selected_mode,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionNameEnum /* buy_spin, respin, spin */,
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
	pub prev_client_command_time: Option<i64> /* 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "8b08e7ff-2dc5-4191-84a7-ea1d9caedccd" */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "415df5f8-1c06-4e37-aa53-6f437c7fe8b8" */,
	pub session_id: String /* "55ea9ad42dac4b82a528beea90e1a81d" */,
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
			prev_request_id: obj.prev_request_id,
			quick_spin: obj.quick_spin,
			request_id: obj.request_id,
			session_id: obj.session_id,
			set_denominator: obj.set_denominator,
			sound: obj.sound,
		}
	}
}

