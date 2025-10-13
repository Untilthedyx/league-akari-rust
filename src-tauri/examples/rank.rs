use tauri_app_demo_lib::lcu::api::rank::Rank;

#[tokio::main]
async fn main() {
    let mut rank = Rank::get_rank_by_puuid("55cc79c4-3d20-535a-9bff-00b1867534d8")
        .await
        .unwrap();
    rank.enrich_cn_info();
    println!("{:#?}", rank);
}
