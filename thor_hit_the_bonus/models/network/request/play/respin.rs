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
	pub autogame: bool /* false */,
	pub command: String /* play */,
	pub mobile: bool /* false */,
	pub portrait: bool /* false */,
	pub prev_client_command_time: i64 /* 147 */,
	pub quick_spin: i64 /* 2 */,
	pub request_id: String /* "3c259adc1e3744bdb6dccdaab7c43db4" */,
	pub session_id: String /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
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
			quick_spin: obj.quick_spin,
			request_id: obj.request_id,
			session_id: obj.session_id,
			sound: obj.sound,
		}
	}
}

