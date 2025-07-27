use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::server::{ContextActionsEnum, LastargsSelectedModeEnum, BsTypeEnum, SpinsBsVEnum, LastbsTypeEnum, BuyspinModesEnum, UserNickEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Achievements {
	pub level: i64 /* 0 */,
	pub level_percent: i64 /* 0 */,
	pub number: i64 /* 0 */,
	pub total_percent: i64 /* 0 */,
}

impl From<server::Achievements> for Achievements {
	fn from(obj: server::Achievements) -> Self {
		Achievements {
			level: obj.level,
			level_percent: obj.level_percent,
			number: obj.number,
			total_percent: obj.total_percent,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bb {
	pub thor_state: i64 /* 1 */,
	pub thor_wrath_spins_left: i64 /* 2, 3, 4 */,
	pub thor_wrath_symbol_pos: Vec<i64> /* [0,0], [0,1], [0,3], [0,4], [1,0], [1,1], [2,0], [2,1], [2,3], [2,4], [3,0], [3,3], [4,0], [4,3], [4,4] */,
	pub thor_wrath_total: i64 /* 1200, 1400, 1600, 1800, 2000, 2200, 2400, 3000, 3200, 3800, 4200, 4600, 5000, 5200 */,
}

impl From<server::Bb> for Bb {
	fn from(obj: server::Bb) -> Self {
		Bb {
			thor_state: obj.thor_state,
			thor_wrath_spins_left: obj.thor_wrath_spins_left,
			thor_wrath_symbol_pos: obj.thor_wrath_symbol_pos,
			thor_wrath_total: obj.thor_wrath_total,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bet20 {
	pub thor_state: i64 /* 0 */,
	pub thor_wrath_spins_left: i64 /* 0 */,
	pub thor_wrath_total: i64 /* 0 */,
}

impl From<server::Bet20> for Bet20 {
	fn from(obj: server::Bet20) -> Self {
		Bet20 {
			thor_state: obj.thor_state,
			thor_wrath_spins_left: obj.thor_wrath_spins_left,
			thor_wrath_total: obj.thor_wrath_total,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastArgs {
	pub bet_factor: i64 /* 10 */,
	pub bet_per_line: i64 /* 20 */,
	pub lines: i64 /* 1 */,
	pub selected_mode: LastargsSelectedModeEnum /* 1, 2, 3 */,
}

impl From<server::LastArgs> for LastArgs {
	fn from(obj: server::LastArgs) -> Self {
		LastArgs {
			bet_factor: obj.bet_factor,
			bet_per_line: obj.bet_per_line,
			lines: obj.lines,
			selected_mode: obj.selected_mode.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bs {
	pub position: i64 /* 0, 1, 2, 3, 4 */,
	pub reel: i64 /* 0, 1, 2, 3, 4 */,
	#[serde(rename = "type")]
	pub bs_type: BsTypeEnum /* collector, infinity, infinity_mystery, infinity_mystery_jp, infinity_wrath, mystery, mystery_jp, regular */,
	pub value: i64 /* 200, 400, 600, 800, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2400, 2600, 2800, 3000, 3200, 3400, 3600, 3800, 4000, 4200, 4400, 4600, 4800, 5000, 5200, 5400, 5600, 5800, 6000, 6200, 6400, 6600, 6800, 7200, 7400, 8200, 10000 */,
}

impl From<server::Bs> for Bs {
	fn from(obj: server::Bs) -> Self {
		Bs {
			position: obj.position,
			reel: obj.reel,
			bs_type: obj.bs_type.into(),
			value: obj.value,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Drawer {
	pub thor_wrath_spins_left: i64 /* 3, 4, 5 */,
}

impl From<server::Drawer> for Drawer {
	fn from(obj: server::Drawer) -> Self {
		Drawer {
			thor_wrath_spins_left: obj.thor_wrath_spins_left,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LastBs {
	pub position: i64 /* 0, 1, 2, 3, 4 */,
	pub reel: i64 /* 0, 1, 2, 3, 4 */,
	#[serde(rename = "type")]
	pub last_bs_type: LastbsTypeEnum /* collector, mystery, mystery_jp, regular */,
	pub value: i64 /* 0, 200, 400, 600, 800, 2000, 4000 */,
}

impl From<server::LastBs> for LastBs {
	fn from(obj: server::LastBs) -> Self {
		LastBs {
			position: obj.position,
			reel: obj.reel,
			last_bs_type: obj.last_bs_type.into(),
			value: obj.value,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NewCollectors {
	pub position: i64 /* 0, 1, 2, 3, 4 */,
	pub reel: i64 /* 0, 1, 2, 3, 4 */,
	#[serde(rename = "type")]
	pub new_collectors_type: String /* collector */,
	pub value: i64 /* 400, 600, 800, 1000, 1200, 1400, 1600, 1800, 2000, 2200, 2400, 2600, 2800, 3000, 3200, 3400, 3600, 3800, 4000, 4200, 4400, 4600, 4800, 5000, 5200, 5400, 5600, 5800, 6000, 6200, 6400, 6600, 6800, 7200, 7400, 8200 */,
}

impl From<server::NewCollectors> for NewCollectors {
	fn from(obj: server::NewCollectors) -> Self {
		NewCollectors {
			position: obj.position,
			reel: obj.reel,
			new_collectors_type: obj.new_collectors_type,
			value: obj.value,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Winlines {
	pub amount: i64 /* 0 */,
	pub line: i64 /* 1 */,
	pub occurrences: i64 /* 5 */,
	pub positions: Vec<Vec<i64>> /* [[0,2],[1,2],[2,2],[3,2],[4,2]] */,
	pub symbol: i64 /* 1 */,
	pub trigger: String /* bonus */,
}

impl From<server::Winlines> for Winlines {
	fn from(obj: server::Winlines) -> Self {
		Winlines {
			amount: obj.amount,
			line: obj.line,
			occurrences: obj.occurrences,
			positions: obj.positions,
			symbol: obj.symbol,
			trigger: obj.trigger,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Spins {
	pub bac: Vec<i64> /* [0,0] */,
	pub bet_per_line: i64 /* 20 */,
	pub bg_type: i64 /* 0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12 */,
	pub board: Vec<Vec<i64>> /* [[0,0,1,0,0],[0,0,1,1,0],[0,0,1,0,0],[1,0,1,0,0],[0,0,1,0,0]] */,
	pub bs: Vec<Bs>,
	pub bs_count: i64 /* 5, 6, 7, 8, 9, 10, 11, 12 */,
	pub bs_new: Vec<Bs>,
	pub bs_new_count: i64 /* 5, 6, 7, 8, 9, 10, 11, 12 */,
	pub bs_v: Vec<Vec<SpinsBsVEnum>> /* [[0,0,800,0,0],[0,0,200,200,0],[0,0,200,0,0],[200,0,200,0,0],[0,0,800,0,0]] */,
	pub bs_values: Vec<Vec<i64>> /* [[0,0,4,0,0],[0,0,1,1,0],[0,0,1,0,0],[1,0,1,0,0],[0,0,4,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub drawer: Option<Drawer>,
	pub is_bought: bool /* true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub is_extra: Option<bool> /* false */,
	pub last_bet: i64 /* 20 */,
	pub last_bs: Vec<LastBs>,
	pub lines: i64 /* 1 */,
	pub mps: Vec<Vec<i64>> /* [[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1],[1,1,1,1,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mult_features: Option<Vec<i64>> /* [10], [11], [12], [6], [7], [8], [9] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mult_pos: Option<Vec<Vec<i64>>> /* [[3,3],[2,3],[0,3],[0,3],[0,2],[4,4],[1,2],[3,1],[1,2],[3,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub new_collectors: Option<Vec<NewCollectors>>,
	pub round_bet: i64 /* 200 */,
	pub round_win: i64 /* 0 */,
	pub selected_mode: LastargsSelectedModeEnum /* 1, 2, 3 */,
	pub temp_board: Vec<Vec<i64>> /* [[0,0,1,0,0],[0,1,0,7,0],[8,0,6,0,0],[0,1,0,1,1],[0,3,1,0,0]] */,
	pub temp_bs_v: Vec<Vec<SpinsBsVEnum>> /* [[0,0,600,0,0],[0,800,0,200,0],[2000,0,0,0,0],[0,400,0,800,200],[0,"minor",600,0,0]] */,
	pub temp_bs_values: Vec<Vec<i64>> /* [[0,0,3,0,0],[0,4,0,1,0],[10,0,0,0,0],[0,2,0,4,1],[0,20,3,0,0]] */,
	pub total_win: i64 /* 0 */,
	pub winlines: Vec<Winlines>,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac,
			bet_per_line: obj.bet_per_line,
			bg_type: obj.bg_type,
			board: obj.board,
			bs: obj.bs.into_iter().map(Into::into).collect(),
			bs_count: obj.bs_count,
			bs_new: obj.bs_new.into_iter().map(Into::into).collect(),
			bs_new_count: obj.bs_new_count,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			drawer: obj.drawer.map(Into::into),
			is_bought: obj.is_bought,
			is_extra: obj.is_extra,
			last_bet: obj.last_bet,
			last_bs: obj.last_bs.into_iter().map(Into::into).collect(),
			lines: obj.lines,
			mps: obj.mps,
			mult_features: obj.mult_features,
			mult_pos: obj.mult_pos,
			new_collectors: obj.new_collectors.map(|vec| vec.into_iter().map(Into::into).collect()),
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			selected_mode: obj.selected_mode.into(),
			temp_board: obj.temp_board,
			temp_bs_v: obj.temp_bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			temp_bs_values: obj.temp_bs_values,
			total_win: obj.total_win,
			winlines: obj.winlines.into_iter().map(Into::into).collect(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub achievements: Achievements,
	pub actions: Vec<ContextActionsEnum> /* bonus_init */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bb: Option<Bb>,
	pub bet20: Bet20,
	pub current: String /* spins */,
	pub last_action: String /* buy_spin */,
	pub last_args: LastArgs,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_win: Option<i64> /* 6200 */,
	pub round_finished: bool /* false */,
	pub spins: Spins,
	pub version: i64 /* 1 */,
}

impl From<server::Context> for Context {
	fn from(obj: server::Context) -> Self {
		Context {
			achievements: obj.achievements.into(),
			actions: obj.actions.into_iter().map(Into::into).collect(),
			bb: obj.bb.map(Into::into),
			bet20: obj.bet20.into(),
			current: obj.current,
			last_action: obj.last_action,
			last_args: obj.last_args.into(),
			last_win: obj.last_win,
			round_finished: obj.round_finished,
			spins: obj.spins.into(),
			version: obj.version,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Data {
	pub quick_spin: bool /* false */,
}

impl From<server::Data> for Data {
	fn from(obj: server::Data) -> Self {
		Data {
			quick_spin: obj.quick_spin,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct OriginData {
	pub autogame: bool /* false */,
	pub data: Data,
	pub feature: bool /* true */,
	pub mobile: bool /* false */,
	pub portrait: bool /* false */,
	pub quick_spin: bool /* false */,
	pub quickspin: i64 /* 2 */,
	pub sound: bool /* false */,
}

impl From<server::OriginData> for OriginData {
	fn from(obj: server::OriginData) -> Self {
		OriginData {
			autogame: obj.autogame,
			data: obj.data.into(),
			feature: obj.feature,
			mobile: obj.mobile,
			portrait: obj.portrait,
			quick_spin: obj.quick_spin,
			quickspin: obj.quickspin,
			sound: obj.sound,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Status {
	pub code: String /* OK */,
}

impl From<server::Status> for Status {
	fn from(obj: server::Status) -> Self {
		Status {
			code: obj.code,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
	pub balance: i64 /* 490000 */,
	pub balance_version: i64 /* 1753038358945 */,
	pub currency: String /* FUN */,
	pub huid: String /* "687d3dfd627d94e85bd38f45" */,
	pub nick: UserNickEnum /* Player 481f841a-566d-4f5d-99b7-c085c96e378e, Player 5a8c32af-ffda-4247-8eb5-05f73d3148e9, Player c80b1aac-2423-4f93-9058-3392e18805de */,
	pub show_balance: bool /* true */,
}

impl From<server::User> for User {
	fn from(obj: server::User) -> Self {
		User {
			balance: obj.balance,
			balance_version: obj.balance_version,
			currency: obj.currency,
			huid: obj.huid,
			nick: obj.nick.into(),
			show_balance: obj.show_balance,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BuySpin {
	pub command: String /* play */,
	pub context: Context,
	pub modes: Vec<BuyspinModesEnum> /* auto, play */,
	pub origin_data: OriginData,
	pub request_id: String /* "84db061b2b5c41b89810510a4c8c283e" */,
	pub session_id: String /* "1753038333859mEuhZklAbJO2lwinJ.RJM4TphE96hbWcWtcHEW" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for BuySpin {
	fn from(obj: server::Server) -> Self {
		BuySpin {
			command: obj.command,
			context: obj.context.into(),
			modes: obj.modes.into_iter().map(Into::into).collect(),
			origin_data: obj.origin_data.into(),
			request_id: obj.request_id,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.into(),
		}
	}
}

