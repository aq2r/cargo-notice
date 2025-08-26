use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use anyhow::Context;
use cargo_metadata::MetadataCommand;
use regex::Regex;

fn get_license_file(path: impl AsRef<Path>) -> anyhow::Result<Vec<PathBuf>> {
    let path = path.as_ref();

    let mut result = vec![];
    for i in path.read_dir()? {
        let i = i?;

        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)license").unwrap());

        let filename = i.file_name().to_string_lossy().to_string();
        if RE.is_match(&filename) {
            let license_path = path.join(filename);
            if license_path.is_file() {
                result.push(license_path);
            } else {
                result.extend(get_license_file(license_path)?);
            }
        }
    }

    Ok(result)
}

pub fn generate() -> anyhow::Result<()> {
    let metadata = MetadataCommand::new().exec()?;

    // 自分自身のクレートを除外する
    let filtered_packages: Vec<_> = metadata
        .packages
        .iter()
        .filter(|p| !metadata.workspace_members.contains(&p.id))
        .collect();

    println!("# ThirdPartyLicenses");
    println!("This is a list of the licenses of libraries that may be included in this program.");
    println!();
    println!(
        "<sub>This file was created by [cargo-notice](https://github.com/aq2r/cargo-notice).</sub>"
    );
    println!();

    println!("## Crate List");
    println!();

    for i in &filtered_packages {
        println!(
            "- [{} v{}](#{}-v{})",
            i.name,
            i.version,
            i.name,
            i.version.to_string().replace(".", "")
        )
    }
    println!();

    println!("## Crate Licenses");

    for i in &filtered_packages {
        match &i.homepage {
            Some(homepage) => println!("### [{} v{}]({})", i.name, i.version, homepage),
            None => println!("### {} v{}", i.name, i.version),
        }

        let author_string = i.authors.join("\n- ");
        if !author_string.is_empty() {
            println!("Author: \n- {author_string}");
            println!();
        }

        if let Some(license) = &i.license {
            println!("License: `{license}`");
        }

        let crate_folder = i
            .manifest_path
            .parent()
            .context("Crate folder was not found")?;

        let license_files = get_license_file(crate_folder)?;
        for j in &license_files {
            let mut file = File::open(j)?;

            let mut s = String::new();
            file.read_to_string(&mut s)?;

            println!("```");
            println!("{s}");
            println!("```");
        }
    }

    Ok(())
}
