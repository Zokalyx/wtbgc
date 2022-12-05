use std::{fs, path::PathBuf, env};
use rand::seq::IteratorRandom;
use serde::Deserialize;
use home;
use json;
use toml;

#[derive(Deserialize)]
struct Config {
    pic_dir: Option<String>,
    allowed_extensions: Option<Vec<String>>,
    profile_file: Option<String>,
    backup_dir: Option<String>,
}

fn main() {
    let mut exe_dir = env::current_exe().unwrap();
    exe_dir.pop();

    let config_file = exe_dir.join("config.toml");
    let config: Config = toml::from_str(&fs::read_to_string(config_file).expect("Couldn't find config.toml!")).unwrap();
    let profile_file = config.profile_file.
        unwrap_or(home::home_dir().unwrap().join("AppData/Local/Packages/Microsoft.WindowsTerminal_8wekyb3d8bbwe/LocalState/settings.json").to_str().unwrap().to_string());
    let pic_dir = config.pic_dir.unwrap_or("backgrounds/".into());
    let backup_dir = config.backup_dir.unwrap_or(exe_dir.to_str().unwrap().to_string());
    let allowed_extensions = config.allowed_extensions.unwrap_or(vec!["gif", "png", "jpg", "jpeg"].into_iter().map(|a| a.to_string()).collect());

    let mut rng = rand::thread_rng();
    let mut new_pic = fs::read_dir(pic_dir).expect("No backgrounds folder found!").into_iter()
        .map(|path| path.unwrap().path())
        .filter(|path| path.extension().is_some())
        .filter(|path| allowed_extensions.contains(&path.extension().unwrap().to_str().unwrap().to_string()))
        .choose(&mut rng).expect("No image found!")
        .canonicalize().unwrap();

    // I don't understand this... but it's needed to write the directory cleanly.
    let new_pic_string = new_pic.to_str().unwrap().to_string();
    if new_pic_string.starts_with("\\\\?\\") {
       new_pic = PathBuf::from(&new_pic_string[4..]);
    }

    let profile = fs::read_to_string(&profile_file).expect("Couldn't find settings.json! Check the config file.");
    fs::write(PathBuf::from(backup_dir).join("backup.json"), &profile).unwrap();

    let mut parsed = json::parse(&profile).unwrap();
    parsed["profiles"]["defaults"]["backgroundImage"] = new_pic.to_str().into();
    let profile = json::stringify_pretty(parsed, 2); {
    fs::write(profile_file, &profile).unwrap();
    }
}
