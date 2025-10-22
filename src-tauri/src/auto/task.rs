// use std::collections::HashMap;
// use std::time::Duration;

// use tokio::time::interval;

// use crate::auto::manager::AUTOMATION_MANAGER;
// use crate::lcu::api::champion_select::{
//     get_champion_select_session, patch_session_action, post_accept_match,
// };
// use crate::lcu::api::lobby::Lobby;
// use crate::lcu::api::lobby::Member;
// use crate::lcu::api::phase::get_phase;
// use crate::lcu::api::summoner::Summoner;
// use crate::lcu::constant::{CHAMPSELECT, LOBBY, MATCHMAKING, READYCHECK};
// use crate::utils::config::{extract_bool, get_config, register_on_change_callback, Value};

// /// 自动接受对局
// async fn auto_accept_match() {
//     let mut ticker = interval(Duration::from_millis(100));

//     loop {
//         ticker.tick().await;

//         match get_phase().await {
//             Ok(phase) if phase == READYCHECK => {
//                 if let Err(e) = post_accept_match().await {
//                     // return Err(format!("接受对局错误:{e}"));
//                 }
//             }
//             Ok(_phase) => {
//                 // println!("当前阶段:{phase}");
//             }
//             Err(e) => {
//                 //
//             }
//         }
//     }
// }

// async fn auto_start_match() {
//     let mut ticker = interval(Duration::from_secs(1));
//     let mut last_search_state = String::new();
//     let mut auto_match_enabled = true;

//     loop {
//         ticker.tick().await;
//         let cur_state = get_phase().await.unwrap();

//         // 状态未变
//         if cur_state == cur_state {
//             continue;
//         }

//         // 匹配 =》 大厅
//         if last_search_state == MATCHMAKING && cur_state == LOBBY {
//             auto_match_enabled = false;
//             last_search_state = cur_state;
//             continue;
//         }

//         // 疑问？？
//         if !auto_match_enabled && cur_state != LOBBY {
//             auto_match_enabled = true;
//             last_search_state = cur_state;
//             continue;
//         }

//         if !auto_match_enabled {
//             last_search_state = cur_state;
//             continue;
//         }

//         last_search_state = cur_state.clone();

//         if cur_state != LOBBY {
//             continue;
//         }

//         let lobby = match Lobby::get_lobby().await {
//             Ok(lobby) => lobby,
//             Err(e) => continue,
//         };

//         if lobby.game_config.is_custom {
//             continue;
//         }

//         match is_leader(&lobby.members).await {
//             Ok(false) => {
//                 continue;
//             }
//             Err(_e) => {
//                 continue;
//             }
//             Ok(true) => {}
//         }

//         // 开始匹配
//         Lobby::post_match_search().await.expect("匹配失败！");

//         // 等待 6s 再循环
//         tokio::time::sleep(Duration::from_secs(6)).await;
//     }
// }

// async fn auto_champion_select() {
//     let mut ticker = interval(Duration::from_secs(2));
//     loop {
//         ticker.tick().await;
//         let cur_phase = get_phase().await.unwrap();

//         if cur_phase != CHAMPSELECT {
//             continue;
//         }

//         select_champion().await.unwrap();
//     }
// }

// async fn select_champion() -> Result<(), String> {
//     let select_session = get_champion_select_session().await?;
//     let cell_id = select_session.local_player_cell_id;

//     let pick_champion_slice = match get_config("settings.auto.pickChampionSlice").await {
//         Ok(Value::Map(m)) => {
//             if let Some(Value::List(list)) = m.get("value") {
//                 list.iter()
//                     .filter_map(|v| match v {
//                         Value::Integer(i) => Some(*i as i32),
//                         _ => None,
//                     })
//                     .collect::<Vec<i32>>()
//             } else {
//                 vec![]
//             }
//         }
//         Ok(Value::List(list)) => list
//             .iter()
//             .filter_map(|v| match v {
//                 Value::Integer(i) => Some(*i as i32),
//                 _ => None,
//             })
//             .collect::<Vec<i32>>(),
//         _ => vec![],
//     };

//     let mut not_select_champion_ids = HashMap::new();

//     // 获取 ban 掉的 champion
//     for action_group in &select_session.actions {
//         if !action_group.is_empty() && action_group[0].action_type == "ban" {
//             for ban in action_group {
//                 if ban.actor_cell_id != cell_id && ban.completed {
//                     not_select_champion_ids.insert(ban.champion_id, true);
//                 }
//             }
//         }
//     }

//     // 获取队友选择的 champion
//     for action_group in &select_session.actions {
//         if !action_group.is_empty() && action_group[0].action_type == "pick" {
//             for pick in action_group {
//                 if pick.actor_cell_id != cell_id && pick.completed {
//                     not_select_champion_ids.insert(pick.champion_id, true);
//                 }
//             }
//         }
//     }

//     let will_ban_champion_id = if pick_champion_slice.is_empty() {
//         1
//     } else {
//         pick_champion_slice
//             .iter()
//             .find(|&&champion_id| !not_select_champion_ids.contains_key(&champion_id))
//             .copied()
//             .unwrap_or(1)
//     };

//     let mut action_id = -1;
//     let mut is_in_process = false;

//     for action_group in &select_session.actions {
//         if !action_group.is_empty() && action_group[0].action_type == "ban" {
//             for ban in action_group {
//                 if ban.actor_cell_id == cell_id && ban.is_in_progress {
//                     action_id = ban.id;
//                     is_in_process = true;
//                     break;
//                 }
//             }
//         }
//     }

//     if action_id != -1 && is_in_process {
//         patch_session_action(action_id, will_ban_champion_id, "ban".to_string(), true).await?;
//     }

//     Ok(())
// }

// async fn auto_ban_champion() {
//     let mut ticker = interval(Duration::from_secs(2));

//     loop {
//         ticker.tick().await;

//         let cur_phase = match get_phase().await {
//             Ok(phase) => phase,
//             Err(_) => continue,
//         };

//         if cur_phase != CHAMPSELECT {
//             continue;
//         }

//         if let Err(e) = ban_champion().await {
//             log::error!("Ban champion error:{}", e);
//         }
//     }
// }

// /// 这里逻辑视乎重复了，可以解耦
// async fn ban_champion() -> Result<(), String> {
//     let select_session = get_champion_select_session().await?;
//     let cell_id = select_session.local_player_cell_id;

//     let ban_champion_slice = match get_config("setting.auto.banChampionSlice").await {
//         Ok(Value::Map(m)) => {
//             if let Some(Value::List(list)) = m.get("value") {
//                 list.iter()
//                     .filter_map(|v| match v {
//                         Value::Integer(i) => Some(*i as i32),
//                         _ => None,
//                     })
//                     .collect()
//             } else {
//                 vec![]
//             }
//         }
//         Ok(Value::List(list)) => list
//             .iter()
//             .filter_map(|v| match v {
//                 Value::Integer(i) => Some(*i as i32),
//                 _ => None,
//             })
//             .collect(),
//         _ => vec![],
//     };

//     let mut not_ban_champion_ids: HashMap<i32, bool> = HashMap::new();
//     let mut have_ban_id = false;

//     for action_group in &select_session.actions {
//         if !action_group.is_empty() && action_group[0].action_type == "ban" {
//             for ban in action_group {
//                 if ban.actor_cell_id == cell_id {
//                     if ban.completed {
//                         return Ok(());
//                     }
//                     have_ban_id = true;
//                 }
//             }
//         }
//     }

//     if !have_ban_id {
//         return Ok(());
//     }

//     // 获取 ban 掉的英雄
//     for action_group in &select_session.actions {
//         if !action_group.is_empty() && action_group[0].action_type == "ban" {
//             for ban in action_group {
//                 if ban.actor_cell_id != cell_id && ban.completed {
//                     not_ban_champion_ids.insert(ban.champion_id, true);
//                 }
//             }
//         }
//     }

//     // 获取队友预选的英雄
//     for action_group in &select_session.actions {
//         if !action_group.is_empty() && action_group[0].action_type == "pick" {
//             for pick in action_group {
//                 if pick.actor_cell_id != cell_id {
//                     not_ban_champion_ids.insert(pick.champion_id, true);
//                 }
//             }
//         }
//     }

//     let will_ban_champion_id = if ban_champion_slice.is_empty() {
//         1
//     } else {
//         ban_champion_slice
//             .iter()
//             .find(|&&champion_id| !not_ban_champion_ids.contains_key(&champion_id))
//             .copied()
//             .unwrap_or(1)
//     };

//     let mut action_id = -1;
//     let mut is_in_process = false;

//     for action_group in &select_session.actions {
//         if !action_group.is_empty() && action_group[0].action_type == "ban" {
//             for ban in action_group {
//                 if ban.actor_cell_id == cell_id && ban.is_in_progress {
//                     action_id = ban.id;
//                     is_in_process = true;
//                     break;
//                 }
//             }
//         }
//     }

//     if action_id != -1 && is_in_process {
//         patch_session_action(action_id, will_ban_champion_id, "ban".to_string(), true).await?;
//     }

//     Ok(())
// }

// async fn init_auto() {
//     let manager = &*AUTOMATION_MANAGER;

//     match get_config("setting.auto.startMatchSwitch").await {
//         Ok(value) => {
//             if let Some(true) = extract_bool(&value) {
//                 manager.start_task("start_match", auto_start_match());
//             }
//         }
//         Err(_e) => {}
//     }

//     match get_config("setting.auto.acceptMatchSwitch").await {
//         Ok(value) => {
//             if let Some(true) = extract_bool(&value) {
//                 manager.start_task("accept_match", auto_accept_match());
//             }
//         }
//         Err(_e) => {}
//     }

//     match get_config("setting.auto.banChampionSwitch").await {
//         Ok(value) => {
//             if let Some(true) = extract_bool(&value) {
//                 manager.start_task("ban_champion", auto_champion_select());
//             } else {
//                 manager.stop_task("ban_champion");
//             }
//         }
//         Err(_e) => {}
//     }
// }

// pub async fn auto() {
//     init_auto().await;

//     register_on_change_callback(|k, v| {
//         let manager = &*AUTOMATION_MANAGER;

//         match k {
//             "settings.auto.startMatchSwitch" => {
//                 if let Some(enabled) = extract_bool(v) {
//                     if enabled {
//                         log::info!("Config: Enabling match automation");
//                         manager.start_task("start_match", auto_start_match());
//                     } else {
//                         log::info!("Config: Disabling match automation");
//                         manager.stop_task("start_match");
//                     }
//                 } else {
//                     log::warn!("Invalid value for startMatchSwitch: {:?}", v);
//                 }
//             }
//             "settings.auto.acceptMatchSwitch" => {
//                 if let Some(enabled) = extract_bool(v) {
//                     if enabled {
//                         log::info!("Config: Enabling accept match automation");
//                         manager.start_task("accept_match", auto_accept_match());
//                     } else {
//                         log::info!("Config: Disabling accept match automation");
//                         manager.stop_task("accept_match");
//                     }
//                 } else {
//                     log::warn!("Invalid value for acceptMatchSwitch: {:?}", v);
//                 }
//             }
//             "settings.auto.pickChampionSwitch" => {
//                 if let Some(enabled) = extract_bool(v) {
//                     if enabled {
//                         log::info!("Config: Enabling champion select automation");
//                         manager.start_task("pick_champion", auto_champion_select());
//                     } else {
//                         log::info!("Config: Disabling champion select automation");
//                         manager.stop_task("pick_champion");
//                     }
//                 } else {
//                     log::warn!("Invalid value for pickChampionSwitch: {:?}", v);
//                 }
//             }
//             "settings.auto.banChampionSwitch" => {
//                 if let Some(enabled) = extract_bool(v) {
//                     if enabled {
//                         log::info!("Config: Enabling champion ban automation");
//                         manager.start_task("ban_champion", auto_ban_champion());
//                     } else {
//                         log::info!("Config: Disabling champion ban automation");
//                         manager.stop_task("ban_champion");
//                     }
//                 } else {
//                     log::warn!("Invalid value for banChampionSwitch: {:?}", v);
//                 }
//             }
//             _ => {
//                 log::debug!("Config changed for unmonitored key: {}", k);
//             }
//         }
//     });
// }

// /// 类似于 utils 操作，不应该放置在这里
// async fn is_leader(members: &[Member]) -> Result<bool, String> {
//     let summoner = Summoner::get_current_summoner()
//         .await
//         .expect("获取当前召唤师失败！");
//     let is_leader = members
//         .iter()
//         .any(|member| member.is_leader && (member.puuid == summoner.puuid));
//     Ok(is_leader)
// }
