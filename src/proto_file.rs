use color_eyre::eyre::Error;
use color_eyre::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::debug;

lazy_static! {
    static ref PROTO_CS_NAMESPACE: Regex =
        Regex::new(r#"^option csharp_namespace = "(.*)";$"#).unwrap();
}

pub fn get_csharp_namespace(proto: &Path) -> Result<String> {
    debug!("Reading proto file {proto:?}");
    let mut proto_src = String::new();
    fs::File::open(proto)?.read_to_string(&mut proto_src)?;

    let namespace_line = proto_src
        .lines()
        .find(|x| PROTO_CS_NAMESPACE.is_match(x))
        .ok_or_else(|| Error::msg(format!("Proto file {proto:?} has no csharp_namespace (1)")))?;

    let proto_cs_namespace = PROTO_CS_NAMESPACE
        .captures(namespace_line)
        .ok_or_else(|| Error::msg(format!("Proto file {proto:?} has no csharp_namespace (2)")))?
        .get(1)
        .ok_or_else(|| Error::msg(format!("Proto file {proto:?} has no csharp_namespace (3)")))?
        .as_str();
    debug!("Found desired namespace {proto_cs_namespace}");

    Ok(proto_cs_namespace.to_string())
}

pub fn get_proto_files(path: &Path) -> Result<Vec<PathBuf>> {
    if !path.exists() || !path.is_dir() {
        return Err(Error::msg(
            "Supplied server path is not a directory or does not exist",
        ));
    }

    let mut buffer = Vec::new();
    for entry in path.read_dir()? {
        let entry = entry?;

        if entry.path().is_dir() {
            let mut result = get_proto_files(&entry.path())?;
            buffer.append(&mut result);
        } else if entry.path().is_file()
            && entry
                .path()
                .extension()
                .unwrap()
                .eq_ignore_ascii_case("proto")
        {
            buffer.push(entry.path());
        }
    }

    Ok(buffer)
}
