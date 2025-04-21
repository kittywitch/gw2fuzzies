mod itemstats;
use {crate::itemstats::ItemStats, log, std::collections::HashMap};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let base_url = "https://api.guildwars2.com/v2/itemstats";
    let item_stats_vec: Vec<u32> = reqwest::get(base_url).await?.json().await?;
    let mut item_stats  = HashMap::new();
    for id in item_stats_vec {
        let url = format!("https://api.guildwars2.com/v2/itemstats/{}", id);
        let item_stat: ItemStats = reqwest::get(url).await?.json().await?;
        println!("{:?}", item_stat.name);
        item_stats.insert(item_stat.name.clone(), item_stat);
    }
    println!("{:?}", item_stats);
    Ok(())
}
