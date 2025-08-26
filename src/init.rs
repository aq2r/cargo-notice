use std::collections::{HashMap, HashSet};

use cargo_metadata::MetadataCommand;
use dialoguer::{MultiSelect, theme::ColorfulTheme};

use crate::{license, notice_toml::NoticeToml};

pub fn init() -> anyhow::Result<()> {
    // get current package dependencies licenses
    let dependencies_licenses = {
        let mut set = HashSet::new();

        let metadata = MetadataCommand::new().exec()?;
        for i in metadata.packages {
            if let Some(license) = &i.license {
                let parsed = license::parse(license);

                for i in parsed {
                    if let license::LicenseParsed::License(l) = i {
                        set.insert(l.to_string());
                    }
                }
            }
        }

        set
    };

    // allow-license
    let options: Vec<_> = dependencies_licenses.iter().collect();
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Select the license you want to allow (Use space to select and enter to confirm)",
        )
        .items(&options)
        .interact()
        .unwrap();

    let selections: Vec<_> = selections.iter().map(|i| options[*i].clone()).collect();

    // write file
    NoticeToml {
        allow_license: selections,
        license_manual: HashMap::new(),
    }
    .write_file_to_default_path()?;

    println!("Initial setup complete.");

    Ok(())
}
