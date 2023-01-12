use crate::proto_file::get_csharp_namespace;
use color_eyre::eyre::Error;
use color_eyre::Result;
use heck::ToPascalCase;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tracing::{debug, info, warn};

pub fn compile_proto(
    proto: &Path,
    include: &Path,
    server_base: &Path,
    cs_out: &Path,
    protoc_path: &Path,
) -> Result<()> {
    let rebased = rebase_path(proto, server_base);
    // Strip file name
    let rebased = rebased.to_string_lossy().replace(
        &proto.file_name().unwrap().to_string_lossy().to_string(),
        "",
    );

    // The target output directory
    // <out_dir>/<relative path to original source>
    let outdir = cs_out.join(rebased);
    debug!("Creating outdir {outdir:?}");
    fs::create_dir_all(&outdir)?;

    let outfile = outdir.join(
        proto
            .file_name() // Get the file name
            .unwrap()
            .to_string_lossy() // Convert to str
            .to_string()
            .to_pascal_case() // Change casing to match C# style
            .replace("Proto", ".cs"), // .proto was changed to Proto, replace with cs extension
    );

    info!("Compiling {proto:?} -> {outfile:?}");

    let proto_cs_namespace = get_csharp_namespace(proto)?;

    debug!("Invoking protoc");
    let protoc_path = protoc_path.to_str().unwrap_or("protoc");

    let output = Command::new(protoc_path)
        .args([
            format!("-I={}", include.to_string_lossy()), // Include dir
            format!("--csharp_out={}", outdir.to_string_lossy()), // C# out dir
            format!("--csharp_opt=base_namespace={proto_cs_namespace}"), // Set the namespace
            proto.to_string_lossy().to_string(),         // The input proto file
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    if !stdout.is_empty() {
        info!("Protoc: {stdout}");
    }

    if !stderr.is_empty() {
        warn!("Protoc: {stderr}");
    }

    if !output.status.success() {
        return Err(Error::msg(format!(
            "Compiling proto file {proto:?} failed: {stderr}"
        )));
    }

    Ok(())
}

fn rebase_path(path: &Path, new_base: &Path) -> PathBuf {
    let rebased = path
        .to_string_lossy() // Convert to str
        .replace(&new_base.to_string_lossy().to_string(), ""); // Replace the base path

    debug!("Rebased path '{path:?}' onto new base '{new_base:?}': '{rebased:?}'");

    PathBuf::from(rebased)
}
