use super::super::super::super::server;

use serde::{Serialize, Deserialize};
use super::super::super::super::server::{ContextActionsEnum, BonusBsVEnum, BonusSelectedModeEnum, BonusinitModesEnum, };

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bac {
	#[serde(rename = "1")]
	pub bac_1: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [6,3], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,2], [8,3], [8,4], [9,0] */,
	#[serde(rename = "2")]
	pub bac_2: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [6,3], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,2], [8,3], [8,4], [9,0] */,
	#[serde(rename = "3")]
	pub bac_3: Vec<i64> /* [1,0], [2,0], [2,1], [3,0], [3,1], [3,2], [4,0], [4,1], [4,2], [4,3], [5,0], [5,1], [5,2], [5,3], [6,0], [6,1], [6,2], [6,3], [7,0], [7,1], [7,2], [7,3], [7,4], [8,0], [8,1], [8,2], [8,3], [8,4], [9,0] */,
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
pub struct BoostValues {
	pub bs_v: i64 /* 60 */,
	pub pos: Vec<i64> /* [2,1] */,
}

impl From<server::BoostValues> for BoostValues {
	fn from(obj: server::BoostValues) -> Self {
		BoostValues {
			bs_v: obj.bs_v,
			pos: obj.pos,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CollectValues {
	pub bs_v: f64 /* 120.0 */,
	pub pos: Vec<i64> /* [3,1] */,
}

impl From<server::CollectValues> for CollectValues {
	fn from(obj: server::CollectValues) -> Self {
		CollectValues {
			bs_v: obj.bs_v,
			pos: obj.pos,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MultiValues {
	pub bs_v: i64 /* 60 */,
	pub mult_value: i64 /* 2, 3, 5 */,
	pub pos: Vec<i64> /* [1,2] */,
}

impl From<server::MultiValues> for MultiValues {
	fn from(obj: server::MultiValues) -> Self {
		MultiValues {
			bs_v: obj.bs_v,
			mult_value: obj.mult_value,
			pos: obj.pos,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bonus {
	pub bac: Bac,
	pub back_to: String /* spins */,
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[10,7,7],[7,7,4],[5,10,10],[4,12,4],[10,10,1]] */,
	pub bonus_game_type: i64 /* 1, 2, 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boost_values: Option<Vec<BoostValues>>,
	pub bs_count: i64 /* 6, 7, 8 */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[10.0,0,0],[0,0,0],[0,20,50.0],[0,120.0,0],[30.0,10.0,0]] */,
	pub bs_values: Vec<Vec<f64>> /* [[0.5,0,0],[0,0,0],[0,1,2.5],[0,6.0,0],[1.5,0.5,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub collect_values: Option<Vec<CollectValues>>,
	pub init_bs_count: bool /* true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_positions: Option<Vec<Vec<BonusBsVEnum>>> /* [[0,0,0],[0,0,"mini"],[0,0,0],[0,0,0],[0,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpot_values: Option<Vec<i64>> /* [200,400,2000] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpots_boost_values: Option<Vec<Vec<i64>>> /* [[0,0,0],[0,0,80],[0,0,0],[0,0,0],[0,0,0]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jackpots_multiplier_values: Option<Vec<Vec<i64>>> /* [[0,0,0],[0,0,2],[0,0,0],[0,0,0],[0,0,0]] */,
	pub last_respin: bool /* false */,
	pub lines: i64 /* 25 */,
	pub lucky_spin_win: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub multi_values: Option<Vec<MultiValues>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_bs_v: Option<Vec<Vec<BonusBsVEnum>>> /* [[10.0,0,0],[0,0,0],[0,20,50.0],[0,0,0],[30.0,10.0,0]] */,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	pub rounds_granted: i64 /* 3 */,
	pub rounds_left: i64 /* 3 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusSelectedModeEnum> /* 1, 2 */,
	pub total_win: i64 /* 0 */,
}

impl From<server::Bonus> for Bonus {
	fn from(obj: server::Bonus) -> Self {
		Bonus {
			bac: obj.bac.into(),
			back_to: obj.back_to,
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bonus_game_type: obj.bonus_game_type,
			bonus_mechanic: obj.bonus_mechanic,
			boost_values: obj.boost_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			bs_count: obj.bs_count,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			collect_values: obj.collect_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			init_bs_count: obj.init_bs_count,
			jackpot_positions: obj.jackpot_positions.map(|vec| vec.into_iter().map(Into::into).collect()),
			jackpot_values: obj.jackpot_values,
			jackpots_boost_values: obj.jackpots_boost_values,
			jackpots_multiplier_values: obj.jackpots_multiplier_values,
			last_respin: obj.last_respin,
			lines: obj.lines,
			lucky_spin_win: obj.lucky_spin_win,
			multi_values: obj.multi_values.map(|vec| vec.into_iter().map(Into::into).collect()),
			origin_bs_v: obj.origin_bs_v.map(|vec| vec.into_iter().map(Into::into).collect()),
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			rounds_granted: obj.rounds_granted,
			rounds_left: obj.rounds_left,
			selected_mode: obj.selected_mode.map(Into::into),
			total_win: obj.total_win,
		}
	}
}

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
pub struct Spins {
	pub bac: Bac,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_pos: Option<Vec<Vec<i64>>> /* [[4,1],[4,0],[3,0],[2,0],[0,2]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bac_win: Option<bool> /* false, true */,
	pub bet_per_line: i64 /* 1 */,
	pub board: Vec<Vec<i64>> /* [[10,7,7],[7,7,4],[5,10,10],[4,12,4],[10,10,1]] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bonus_mechanic: Option<Vec<i64>> /* [1,2,3], [1,2], [1,3], [1], [2,3], [2], [3] */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bs_count: Option<i64> /* 6 */,
	pub bs_v: Vec<Vec<BonusBsVEnum>> /* [[10.0,0,0],[0,0,0],[0,20,50.0],[0,120.0,0],[30.0,10.0,0]] */,
	pub bs_values: Vec<Vec<f64>> /* [[0.5,0,0],[0,0,0],[0,1,2.5],[0,6.0,0],[1.5,0.5,0]] */,
	pub lines: i64 /* 25 */,
	pub lucky_spin_win: bool /* false, true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin_board: Option<Vec<Vec<i64>>> /* [[5,5,1],[4,4,13],[9,4,4],[4,4,4],[6,4,4]] */,
	pub round_bet: i64 /* 20 */,
	pub round_win: i64 /* 0 */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub selected_mode: Option<BonusSelectedModeEnum> /* 1, 2 */,
}

impl From<server::Spins> for Spins {
	fn from(obj: server::Spins) -> Self {
		Spins {
			bac: obj.bac.into(),
			bac_pos: obj.bac_pos,
			bac_win: obj.bac_win,
			bet_per_line: obj.bet_per_line,
			board: obj.board,
			bonus_mechanic: obj.bonus_mechanic,
			bs_count: obj.bs_count,
			bs_v: obj.bs_v.into_iter().map(|inner_vec| {inner_vec.into_iter().map(Into::into).collect()}).collect(),
			bs_values: obj.bs_values,
			lines: obj.lines,
			lucky_spin_win: obj.lucky_spin_win,
			origin_board: obj.origin_board,
			round_bet: obj.round_bet,
			round_win: obj.round_win,
			selected_mode: obj.selected_mode.map(Into::into),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
	pub actions: Vec<ContextActionsEnum> /* respin */,
	pub bonus: Bonus,
	pub current: String /* bonus */,
	pub last_action: String /* bonus_init */,
	pub last_args: LastArgs,
	pub last_win: i64 /* 40 */,
	pub round_finished: bool /* false */,
	pub spins: Spins,
	pub version: i64 /* 1 */,
}

impl From<server::Context> for Context {
	fn from(obj: server::Context) -> Self {
		Context {
			actions: obj.actions.into_iter().map(Into::into).collect(),
			bonus: obj.bonus.into(),
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
pub struct OriginData {
	pub autogame: bool /* true */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub command: Option<String> /* play */,
	pub feature: bool /* true */,
	pub mobile: String /* 0 */,
	pub portrait: bool /* false */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prev_request_id: Option<String> /* "afe80340-2666-42ea-987e-7893c5ef49ba" */,
	pub quickspin: i64 /* 2 */,
	pub set_denominator: i64 /* 1 */,
	pub sound: bool /* false */,
}

impl From<server::OriginData> for OriginData {
	fn from(obj: server::OriginData) -> Self {
		OriginData {
			autogame: obj.autogame,
			command: obj.command,
			feature: obj.feature,
			mobile: obj.mobile,
			portrait: obj.portrait,
			prev_request_id: obj.prev_request_id,
			quickspin: obj.quickspin,
			set_denominator: obj.set_denominator,
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
	pub balance: i64 /* 99735 */,
	pub balance_version: i64 /* 27 */,
	pub currency: String /* FUN */,
	pub huid: String /* "demo-106758a99a3346fba872f844aa187a8c" */,
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
pub struct BonusInit {
	pub command: String /* play */,
	pub context: Context,
	pub modes: Vec<BonusinitModesEnum> /* auto, freebet, play */,
	pub origin_data: OriginData,
	pub request_id: String /* "8c77e740-6385-494c-9dce-9b10737ee42e" */,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub roundnum: Option<String> /* "2505171000004313117" */,
	pub session_id: String /* "04d1923972bc43a9a629302732728d65" */,
	pub status: Status,
	pub user: User,
}

impl From<server::Server> for BonusInit {
	fn from(obj: server::Server) -> Self {
		BonusInit {
			command: obj.command,
			context: obj.context.into(),
			modes: obj.modes.into_iter().map(Into::into).collect(),
			origin_data: obj.origin_data.into(),
			request_id: obj.request_id,
			roundnum: obj.roundnum,
			session_id: obj.session_id,
			status: obj.status.into(),
			user: obj.user.into(),
		}
	}
}

