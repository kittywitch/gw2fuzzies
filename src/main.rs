mod api;
use {
    crate::api::*,
    futures_util::{SinkExt, StreamExt, TryStreamExt},
    log,
    std::{
        env::{current_dir, current_exe},
        path::PathBuf,
        sync::Arc,
    },
    strum_macros::Display,
    tokio::{
        fs::{create_dir_all, File},
        sync::Semaphore,
        task::JoinSet,
    },
    clap::{
        Parser,
        Subcommand,
    },
    tokio_into_sink::IntoSinkExt as _,
};

async fn get_kind_ids(kind: &String) -> anyhow::Result<Vec<u32>> {
    let base_url = format!("https://api.guildwars2.com/v2/{kind}");
    let ids: Vec<u32> = reqwest::get(base_url).await?.json().await?;
    Ok(ids)
}

async fn download_kind_json(simultaneous_limit: usize, kind: String) -> anyhow::Result<()> {
    log::info!(
        "Preparing to download all {kind} with a simultaneous download limit of {simultaneous_limit}!"
    );

    let executable_folder = match current_exe()?.parent() {
        Some(folder) => folder.to_path_buf(),
        None => current_dir()?.to_path_buf(),
    };

    let ids = get_kind_ids(&kind).await?;
    let mut set = JoinSet::new();
    let path_str = format!("{kind}/");
    let path = executable_folder.join(path_str);

    log::info!(
        "Preparing to download all {} {kind} with a simultaneous download limit of {simultaneous_limit} to {:?}",
        ids.len(),
        path
    );
    create_dir_all(&path).await?;

    let semaphore = Arc::new(Semaphore::new(simultaneous_limit));

    for id in ids {
        let permit = semaphore.clone().acquire_owned().await?;
        let kind = kind.clone();
        let path = path.clone();
        let url = format!("https://api.guildwars2.com/v2/{kind}/{id}");
        set.spawn(async move {
            log::debug!("Downloading {kind} {id}!");
            let download = reqwest::get(url).await?;
            let filename = format!("{id}.json");
            let filepath = path.join(filename);
            let file = File::create(filepath).await?;
            let mut file_sink = file.into_sink().sink_map_err(anyhow::Error::from);
            let bytes_stream = download.bytes_stream().map_err(anyhow::Error::from);
            bytes_stream.forward(&mut file_sink).await?;
            file_sink.close().await?;
            log::debug!("Downloaded {kind} {id}!");
            drop(permit);
            Ok::<(), anyhow::Error>(())
        });
    }

    while let Some(res) = set.join_next().await {
        res??;
    }

    log::info!("Download of {kind} complete!");
    Ok(())
}

#[derive(Display)]
#[allow(dead_code)]
#[strum(serialize_all = "snake_case")]
enum Api {
    Traits,
    Skills,
    ItemStats,
    Specializations,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    verb: Verb,
}

#[derive(Parser, Debug)]
struct Download {
    /// Provide a GW2 API endpoint, e.g. traits, skills, itemstats, specializations.
    kind: String,
    #[arg(
            long,
            short,
            require_equals = false,
            value_name = "limit",
            num_args = 0..=1,
            default_value_t = 100,
            default_missing_value = "100",
    )]
    /// Simultaneous download limit
    limit: usize,
}

#[derive(Subcommand, Debug)]
enum Verb {
    #[command(arg_required_else_help = true)]
    Download(Download),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    match args.verb {
        Verb::Download (download) => download_kind_json(download.limit, download.kind).await?,
    }

    Ok(())
}
