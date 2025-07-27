use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	pub bet_factor: i64 /* 20 */,
	pub bet_per_line: i64 /* 150 */,
	pub lines: i64 /* 25 */,
	pub selected_mode: String /* 1 */,
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
pub struct Init {
	pub action: Action,
	pub autogame: bool /* true */,
	pub command: String /* play */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "d3752a3a-4d16-423c-a34e-03a3209459bd" */,
	pub session_id: String /* "835130fe7a4345358ea53b23e8a004d5" */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false */,
}

impl From<client::Client> for Init {
	fn from(obj: client::Client) -> Self {
		Init {
			action: obj.action.into(),
			autogame: obj.autogame,
			command: obj.command,
			mobile: obj.mobile,
			portrait: obj.portrait,
			quick_spin: obj.quick_spin,
			request_id: obj.request_id,
			session_id: obj.session_id,
			set_denominator: obj.set_denominator,
			sound: obj.sound,
		}
	}
}

