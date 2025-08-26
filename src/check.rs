use std::sync::LazyLock;

use cargo_metadata::MetadataCommand;
use colored::Colorize as _;
use regex::Regex;

use crate::{
    license::{self},
    notice_toml::NoticeToml,
};

fn colorize_expression(expr: &str, list: &[&str]) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"[\w\-\_\.]+ WITH [\w\-\_\.]+|[\w\-\_\.]+|[\(\)]|AND|OR|and|or").unwrap()
    });

    let result = RE.replace_all(expr, |caps: &regex::Captures| {
        let token = &caps[0];

        if token.eq_ignore_ascii_case("AND")
            || token.eq_ignore_ascii_case("OR")
            || token == "("
            || token == ")"
        {
            token.to_string()
        } else if list.contains(&token) {
            token.green().to_string()
        } else {
            token.red().to_string()
        }
    });

    result.into_owned()
}

pub fn check() -> anyhow::Result<()> {
    let notice_toml = match NoticeToml::read_file_from_default_path() {
        Ok(toml) => toml,
        Err(_) => anyhow::bail!("Initial setup is not complete. Please use the init command."),
    };

    let allow_list: Vec<&str> = notice_toml
        .allow_license
        .iter()
        .map(|s| s.as_str())
        .collect();

    let metadata = MetadataCommand::new().exec()?;

    // 自分自身のクレートを除外する
    let filtered_packages: Vec<_> = metadata
        .packages
        .iter()
        .filter(|p| !metadata.workspace_members.contains(&p.id))
        .collect();

    for i in filtered_packages {
        match (&i.license, notice_toml.license_manual.get(i.name.as_str())) {
            (None, None) => {
                println!(
                    "\
                    Checking - {} v{}\n\
                    LICENSE: {}\
                    ",
                    i.name,
                    i.version,
                    "None".red(),
                );
                anyhow::bail!(
                    "\
                    License check Failed: \n\
                    Crate Name: {}\n\
                    License: None\
                    ",
                    i.name,
                );
            }
            (license, manual_license) => {
                let some_license = 'a: {
                    if let Some(l) = manual_license {
                        break 'a Some(l);
                    }

                    if let Some(l) = license {
                        break 'a Some(l);
                    }

                    None
                }
                .unwrap();

                println!(
                    "\
                    {} v{}: {}\
                    ",
                    i.name,
                    i.version,
                    colorize_expression(some_license, &allow_list)
                );
                if !license::license_check(some_license, &allow_list) {
                    anyhow::bail!(
                        "\
                    License check Failed: \n\
                    Crate Name: {}\n\
                    License: {}\n\
                    ",
                        i.name,
                        some_license
                    )
                }
            }
        }
    }

    println!("License Check Complete!");
    Ok(())
}
