use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{ParamsSelectedModeEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	pub bet_factor: i64 /* 10 */,
	pub bet_per_line: i64 /* 20 */,
	pub lines: i64 /* 1 */,
	pub selected_mode: ParamsSelectedModeEnum /* 1, 2, 3 */,
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
	pub autogame: bool /* false */,
	pub command: String /* play */,
	pub mobile: bool /* false */,
	pub portrait: bool /* false */,
	pub prev_client_command_time: i64 /* 29 */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "84db061b2b5c41b89810510a4c8c283e" */,
	pub session_id: String /* "1753038333859mEuhZklAbJO2lwinJ.RJM4TphE96hbWcWtcHEW" */,
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
			quick_spin: obj.quick_spin,
			request_id: obj.request_id,
			session_id: obj.session_id,
			sound: obj.sound,
		}
	}
}

