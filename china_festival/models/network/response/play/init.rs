use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::server::{ContextActionsEnum, InitModesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
}

impl From<server::LastArgs> for LastArgs {
	fn from(obj: server::LastArgs) -> Self {
		LastArgs {
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bac {
	#[serde(rename = "1")]
	pub bac_1: Vec<i64> /* [3,0] */,
	#[serde(rename = "2")]
	pub bac_2: Vec<i64> /* [3,0] */,
	#[serde(rename = "3")]
	pub bac_3: Vec<i64> /* [3,0] */,
}

impl From<server::Bac> for Bac {
	fn from(obj: server::Bac) -> Self {
		Bac {
			bac_1: obj.bac_1,
			bac_2: obj.bac_2,
			bac_3: obj.bac_3,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	pub bac: Bac,
	pub bet_per_line: i64 /* 12 */,
	pub board: Vec<Vec<i64>> /* [[2,2,1],[3,6,6],[9,9,9],[2,8,8],[5,2,2]] */,
	pub bs_count: i64 /* 0 */,
	pub is_lucky_spin: bool /* false */,
	pub lines: i64 /* 25 */,
	pub round_bet: i64 /* 240 */,
	pub round_win: i64 /* 0 */,
	pub total_win: i64 /* 0 */,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac.into(),
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bs_count: obj.bs_count,
			is_lucky_spin: obj.is_lucky_spin,
			lines: obj.lines,
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			total_win: obj.total_win,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ContextActionsEnum> /* buy_spin, spin */,
	pub current: String /* spins */,
	pub last_action: String /* init */,
	pub last_args: LastArgs,
	pub round_finished: bool /* true */,
	pub spins: Spins,
	pub version: i64 /* 1 */,
}

impl From<server::Context> for Context {
	fn from(obj: server::Context) -> Self {
		Context {
			actions: obj.actions.into_iter().map(Into::into).collect(),
			current: obj.current,
			last_action: obj.last_action,
			last_args: obj.last_args.into(),
			round_finished: obj.round_finished,
			spins: obj.spins.into(),
			version: obj.version,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: String /* FUNDS_EXCEED */,
	#[serde(rename = "type")]
	pub status_type: String /* exceed */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code,
			status_type: obj.status_type,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 100000 */,
	pub balance_version: i64 /* 2 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-5a8e338b13544c9d918097fb87760b21" */,
	pub show_balance: bool /* true */,
}

impl From<server::User> for User {
	fn from(obj: server::User) -> Self {
		User {
			balance: obj.balance,
			balance_version: obj.balance_version,
			currency: obj.currency,
			huid: obj.huid,
			show_balance: obj.show_balance,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Init {
	pub command: String /* play */,
	pub context: Context,
	pub modes: Vec<InitModesEnum> /* auto, play */,
	pub request_id: String /* "d3752a3a-4d16-423c-a34e-03a3209459bd" */,
	pub session_id: String /* "835130fe7a4345358ea53b23e8a004d5" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for Init {
	fn from(obj: server::Server) -> Self {
		Init {
			command: obj.command,
			context: obj.context.into(),
			modes: obj.modes.into_iter().map(Into::into).collect(),
			request_id: obj.request_id,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.into(),
		}
	}
}

