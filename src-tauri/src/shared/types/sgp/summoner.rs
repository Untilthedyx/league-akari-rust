use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SgpSummoner {
    pub id: i64,
    pub puuid: String,
    pub account_id: i64,
    pub name: String,
    pub internal_name: String,
    pub profile_icon_id: i64,
    pub level: i64,
    pub exp_points: i64,
    pub level_and_xp_version: i64,
    pub revision_id: i64,
    pub revision_date: i64,
    pub last_game_date: i64,
    pub name_change_flag: bool,
    pub unnamed: bool,
    pub privacy: String,
    pub exp_to_next_level: i64,
}
