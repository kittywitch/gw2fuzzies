mod itemstats;
use {crate::itemstats::ItemStats, log};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://api.guildwars2.com/v2/itemstats/1011";
    let item_stats: ItemStats = reqwest::get(url).await?.json().await?;
    log::info!("{:?}", item_stats);
    println!("{:?}", item_stats);
    let attribute = item_stats.attributes.iter().find(|attr| attr.attribute == "Power");
    println!("{:?}", attribute);
    Ok(())
}
