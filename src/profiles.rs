use std::path::{Path, PathBuf};

use ini::Ini;

use crate::error::{Error, Result};

// [x] locate the file
// [x] parse it
// [x] resolve relative paths
// [x] apply the Install-section default
// [x] match names case-insensitively

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub path: PathBuf, // already resolved
}

pub fn find<'a>(name: &str, profiles: &'a [Profile]) -> Result<&'a Profile> {
    profiles
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| Error::UnknownProfile {
            name: name.to_string(),
            available: profiles.iter().map(|p| p.name.clone()).collect(),
        })
}

pub fn load(zen_dir: &Path) -> Result<(Vec<Profile>, Option<usize>)> {
    let ini_path = zen_dir.join("profiles.ini");
    let text = std::fs::read_to_string(&ini_path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => Error::IniNotFound(ini_path.clone()),
        _ => Error::Io(e),
    })?;
    parse(&text, zen_dir)
}

pub fn parse(text: &str, zen_dir: &Path) -> Result<(Vec<Profile>, Option<usize>)> {
    // don't interpret backslash escapes or strip surrounding quotes from names and paths
    let opt = ini::ParseOption {
        enabled_escape: false,
        enabled_quote: false,
        ..Default::default()
    };

    let ini = Ini::load_from_str_opt(text, opt).map_err(|e| Error::MalformedIni(e.to_string()))?;

    let mut default_profile: Option<PathBuf> = None;
    let mut legacy_default: Option<usize> = None;
    let mut profiles = Vec::new();
    for (section, props) in ini.iter() {
        let Some(section) = section else { continue };
        if section.starts_with("Install") {
            let Some(default_value) = props.get("Default") else {
                continue;
            };
            // if several [Install] sections, keep first
            if default_profile.is_none() {
                default_profile = Some(zen_dir.join(default_value));
            }
            continue;
        }
        if !section.starts_with("Profile") {
            continue;
        }
        let (Some(name), Some(path)) = (props.get("Name"), props.get("Path")) else {
            continue; // for now just ignore bad entries
        };
        let path = if props.get("IsRelative") == Some("1") {
            zen_dir.join(path)
        } else {
            PathBuf::from(path)
        };
        // legacy default marker, used only when no Install section matches
        if legacy_default.is_none() && props.get("Default") == Some("1") {
            legacy_default = Some(profiles.len());
        }
        profiles.push(Profile {
            name: name.to_string(),
            path,
        });
    }

    // prefer the Install section's default, fall back to `Default=1`, else no default
    let default_index = default_profile
        .and_then(|path| profiles.iter().position(|p| p.path == path))
        .or(legacy_default);

    Ok((profiles, default_index))
}
