use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub const NOTICETOML_PATH: &str = "./cargo-notice.toml";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NoticeToml {
    pub allow_license: Vec<String>,
    pub license_manual: HashMap<String, String>,
}

impl NoticeToml {
    pub fn read_file(path: impl AsRef<Path>) -> anyhow::Result<NoticeToml> {
        let mut file = File::open(path)?;

        let mut s = String::new();
        file.read_to_string(&mut s)?;

        Ok(toml::from_str(&s)?)
    }

    pub fn read_file_from_default_path() -> anyhow::Result<NoticeToml> {
        let notice_toml = Self::read_file(NOTICETOML_PATH)?;
        Ok(notice_toml)
    }

    pub fn write_file(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let mut file = File::create(path)?;
        let s = toml::to_string(self)?;

        file.write_all(s.as_bytes())?;

        Ok(())
    }

    pub fn write_file_to_default_path(&self) -> anyhow::Result<()> {
        self.write_file(NOTICETOML_PATH)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read, path::PathBuf};

    use super::*;

    #[ignore]
    #[test]
    fn test_loadtoml() {
        let s = {
            let path = PathBuf::from("./sample/template.toml");
            let mut file = File::open(&path).unwrap();
            let mut s = String::new();
            file.read_to_string(&mut s).unwrap();

            s
        };

        let parsed: NoticeToml = toml::from_str(&s).unwrap();
        dbg!(parsed);
    }
}
