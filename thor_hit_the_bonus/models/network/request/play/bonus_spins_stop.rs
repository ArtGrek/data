use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{ActionNameEnum, ParamsSelectedModeEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_factor: Option<i64> /* 10 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bet_per_line: Option<i64> /* 20 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lines: Option<i64> /* 1 */,
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
	pub name: ActionNameEnum /* bonus_spins_stop, buy_spin */,
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
pub struct BonusSpinsStop {
	pub action: Action,
	pub autogame: bool /* false */,
	pub command: String /* play */,
	pub mobile: bool /* false */,
	pub portrait: bool /* false */,
	pub prev_client_command_time: i64 /* 155 */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "de70abbd38cf4a678bd313fe15383683" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
	pub sound: bool /* false */,
}

impl From<client::Client> for BonusSpinsStop {
	fn from(obj: client::Client) -> Self {
		BonusSpinsStop {
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

