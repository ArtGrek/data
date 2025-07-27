use super::super::super::super::client;

use serde::{Serialize, Deserialize};
use super::super::super::super::client::{};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Params {
	pub bet_per_line: i64 /* 20 */,
	pub lines: i64 /* 1 */,
}

impl From<client::Params> for Params {
	fn from(obj: client::Params) -> Self {
		Params {
			bet_per_line: obj.bet_per_line,
			lines: obj.lines,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: String /* spin */,
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
pub struct Spin {
	pub action: Action,
	pub autogame: bool /* false */,
	pub command: String /* play */,
	pub mobile: bool /* false */,
	pub portrait: bool /* false */,
	pub prev_client_command_time: i64 /* 19 */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "36abc13f56974c1ba4f5957f7e8c98ea" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
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
			quick_spin: obj.quick_spin,
			request_id: obj.request_id,
			session_id: obj.session_id,
			sound: obj.sound,
		}
	}
}

