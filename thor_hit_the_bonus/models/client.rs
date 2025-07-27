use serde::{Serialize, Deserialize};
use strum_macros::Display;

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ActionNameEnum {
	#[default]
	#[serde(rename = "bonus_init")]
	BonusInit,
	#[serde(rename = "bonus_spins_stop")]
	BonusSpinsStop,
	#[serde(rename = "buy_spin")]
	BuySpin,
	#[serde(rename = "respin")]
	Respin,
	#[serde(rename = "spin")]
	Spin,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ParamsSelectedModeEnum {
	#[default]
	#[serde(rename = "1")]
	Enum1,
	#[serde(rename = "2")]
	Enum2,
	#[serde(rename = "3")]
	Enum3,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display, Default)]
pub enum ClientCommandEnum {
	#[default]
	#[serde(rename = "login")]
	Login,
	#[serde(rename = "play")]
	Play,
	#[serde(rename = "start")]
	Start,
	#[serde(rename = "sync")]
	Sync,
}

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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Action {
	pub name: ActionNameEnum /* bonus_init, bonus_spins_stop, buy_spin, respin, spin */,
	pub params: Params,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Client {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub action: Option<Action>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub autogame: Option<bool> /* false */,
	pub command: ClientCommandEnum /* login, play, start, sync */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub huid: Option<String> /* "686e6f522c7c80483b132b54" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String> /* en */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mobile: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mode: Option<String> /* play */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub portrait: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_client_command_time: Option<i64> /* 19 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quick_spin: Option<i64> /* 2 */,
	pub request_id: String /* "816eb35963c141d684121e4f5d1557e1" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String> /* "17520679221969UH15ouuh3xFUSvXY.EmVaz7x07pImki9byd2v" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sound: Option<bool> /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String> /* "537c5edd-428c-4a00-8e3c-41e899ce4166" */,
}

