use std::{collections::{HashMap, HashSet}, fs::exists, path::Path};

//mod api;
use {
    //crate::api::*,
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

use anyhow::anyhow;
use glam::Vec2;
use glamour::{Point2, Unit};
use glob::Paths;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;

struct MapSpace;
impl Unit for MapSpace {
    type Scalar = f32;
}

struct LocalSpace;
impl Unit for LocalSpace {
    type Scalar = f32;
}

type MapPoint = Point2<MapSpace>;
type LocalPoint = Point2<LocalSpace>;

#[derive(Deserialize,Serialize)]
enum MapType {
    BlueHome,
    Center,
    EdgeOfTheMists,
    GreenHome,
    Instance,
    JumpPuzzle,
    Public,
    Pvp,
    RedHome,
    Tutorial,
    Unknown
}

#[derive(Deserialize,Serialize)]
struct Map {
    id: usize,
    name: String,
    min_level: isize,
    max_level: isize,
    default_floor: isize,
    #[serde(alias = "type")]
    kind: MapType,
    floors: Vec<isize>,
    region_id: Option<isize>,
    region_name: Option<String>,
    continent_id: Option<isize>,
    continent_name: Option<String>,
    map_rect: [Vec2; 2],// [LocalPoint; 2],
    continent_rect: [Vec2; 2], // [MapPoint; 2],
}


impl Map {
    pub async fn do_map_ratio() -> anyhow::Result<()> {
        let executable_folder = match current_exe()?.parent() {
            Some(folder) => folder.to_path_buf(),
            None => current_dir()?.to_path_buf(),
        };
        let path_str = format!("maps/");
        let path = executable_folder.join(path_str);
        let maps = Self::load_many(&path, 100).await?;
        Self::get_ratios(maps).await;
        Ok(())
    }
    pub fn glob() -> String {
        "**/*.json".to_string()
    }

    pub fn path_glob(path: &Path) -> PathBuf {
        path.join(&Self::glob())
    }

    pub fn get_paths(path: &Path) -> anyhow::Result<Paths> {
        let pathbuf_glob = Self::path_glob(path);

        let path_glob_str = pathbuf_glob.to_str()
            .ok_or_else(|| anyhow!("Timer file loading path glob unparseable for {path:?}"))?;
            Ok(glob::glob(path_glob_str)?)
    }
    pub async fn load_many(load_dir: &Path, simultaneous_limit: usize) -> anyhow::Result<Vec<Self>> {
        log::debug!("Beginning load_many for {load_dir:?} with a simultaneous open limit of {simultaneous_limit}.");
        let mut set = JoinSet::new();
        let semaphore = Arc::new(Semaphore::new(simultaneous_limit));
        let mut paths = Self::get_paths(load_dir)?;
        while let Some(path) = paths.next() {
            let permit = semaphore.clone().acquire_owned().await?;
            let path = path?.clone();
            set.spawn(async move {
                let map_file = Self::load(&path).await?;
                drop(permit);
                Ok::<Self, anyhow::Error>(map_file)
            });

        }
        let mut map_files = Vec::new();
        let (mut join_errors, mut load_errors): (usize, usize) = (0, 0);
        while let Some(map_file) = set.join_next().await {
            match map_file {
                Ok(res) => match res {
                    Ok(map_file) => {
                        map_files.push(map_file);
                    },
                    Err(err) => {
                        load_errors += 1;
                        log::error!("map load_many error for {load_dir:?}: {err}");
                    },
                },
                Err(err) => {
                    join_errors += 1;
                    log::error!("map load_many join error for {load_dir:?}: {err}");
                },
            }
        }
        log::debug!(
            "Finished load_many for {load_dir:?}: {} succeeded, {join_errors} join errors, {load_errors} other errors.",
            map_files.len()
        );
        Ok(map_files)
    }

    pub async fn load(path: &Path) -> anyhow::Result<Self> {
        log::debug!("Attempting to load the map file at \"{path:?}\".");
        let mut file_data = read_to_string(path).await?;
        json_strip_comments::strip(&mut file_data)?;
        let mut data: Self = serde_json::from_str(&file_data)?;
        log::debug!("Successfully loaded the map file at \"{path:?}\".");
        Ok(data)
    }
    async fn get_ratios(maps: Vec<Self>) {
        let mut ratios = HashMap::new();
        let mut occurrences: HashMap<String, Vec<String>> = HashMap::new();
        for map in maps {
            let local_diff = map.map_rect[0] - map.map_rect[1];
            let continent_diff = map.continent_rect[0] - map.continent_rect[1];
            let ratio = local_diff / continent_diff;
            let entry = occurrences.entry(format!("{:?}", ratio)).or_default();
            //*entry += 1;
            entry.push(map.name.clone());
            ratios.insert(map.name.clone(), ratio);
        }

        for (name, ratio) in ratios {
            if ratio != Vec2::splat(24.0) {
                log::info!("{name}: {ratio}");
            }
            if format!("{ratio:?}") == "Vec2(58.909092, 31.5)" {
                log::info!("gadooks {name}");
            }
        }

        for (ratio, names) in occurrences {
            if ratio != "Vec2(24.0, 24.0)" {
                log::info!("{ratio}: {}", names.join(", "));
            }
        }
    }
}

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
            if exists(path.join(format!("{id}.json")))? {
                log::info!("File for {id} already exists. Skipping!");
                return Ok::<(), anyhow::Error>(())
            }
            log::debug!("Downloading {kind} {id}!");
            let download = reqwest::get(url).await?.error_for_status()?;
            let status = download.status();
            if status == StatusCode::from_u16(429)? {
                return Err(anyhow!("429 error, too many requests :("));
            }
            let filename = format!("{id}.json");
            let filepath = path.join(filename);
            let file = File::create(filepath).await?;
            let mut file_sink = file.into_sink().sink_map_err(anyhow::Error::from);
            let bytes_stream = download.bytes_stream().map_err(anyhow::Error::from);
            bytes_stream.forward(&mut file_sink).await?;
            file_sink.close().await?;
            log::debug!("Downloaded {kind} {id}, {status:?}!");
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
    MapRatio,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let args = Args::parse();

    match args.verb {
        Verb::Download (download) => download_kind_json(download.limit, download.kind).await?,
        Verb::MapRatio => Map::do_map_ratio().await?,
    }

    Ok(())
}
