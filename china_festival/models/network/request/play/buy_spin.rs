use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{ParamsSelectedModeEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	pub bet_factor: i64 /* 20 */,
	pub bet_per_line: i64 /* 1, 150 */,
	pub lines: i64 /* 25 */,
	pub selected_mode: ParamsSelectedModeEnum /* 1, 2 */,
}

impl From<client::Params> for Params {
	fn from(obj: client::Params) -> Self {
		Params {
			bet_factor: obj.bet_factor,
			bet_per_line: obj.bet_per_line,
			lines: obj.lines,
			selected_mode: obj.selected_mode.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: String /* buy_spin */,
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
pub struct BuySpin {
	pub action: Action,
	pub autogame: bool /* true */,
	pub command: String /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 295 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "cba5c16b-0ff4-4af2-9c38-a1092bec7128" */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "a30dde91-5145-4a04-a233-11772e89cf8f" */,
	pub session_id: String /* "54d657fdddea4c76800b216371ea868e" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false, true */,
}

impl From<client::Client> for BuySpin {
	fn from(obj: client::Client) -> Self {
		BuySpin {
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

