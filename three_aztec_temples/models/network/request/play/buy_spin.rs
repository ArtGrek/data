use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{ParamsSelectedModeEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	pub bet_factor: i64 /* 20 */,
	pub bet_per_line: i64 /* 1 */,
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
	pub prev_client_command_time: Option<i64> /* 282 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "52f719fc-f5b1-462d-9916-436a954394b8" */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "b99dc040-5942-4f54-a639-3421242af4ac" */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false */,
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

