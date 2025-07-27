use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{ActionNameEnum, ParamsSelectedModeEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 20 */,
	pub bet_per_line: i64 /* 1 */,
	pub lines: i64 /* 1 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<ParamsSelectedModeEnum> /* 1, 2, 3 */,
}

impl From<client::Params> for Params {
	fn from(obj: client::Params) -> Self {
		Params {
			bet_factor: obj.bet_factor,
			bet_per_line: obj.bet_per_line,
			lines: obj.lines,
			selected_mode: obj.selected_mode.map(Into::into),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionNameEnum /* buy_spin, spin */,
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
pub struct Spin {
	pub action: Action,
	pub autogame: bool /* true */,
	pub command: String /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 72440 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "c684aeb9-2d0a-48cc-a741-397ebda1956a" */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "d9c0c078-4ed3-4611-8dda-946f0801a8c2" */,
	pub session_id: String /* "577040c7bf0b4dc18036a41bc4527fb7" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false */,
}

impl From<client::Client> for Spin {
	fn from(obj: client::Client) -> Self {
		Spin {
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

