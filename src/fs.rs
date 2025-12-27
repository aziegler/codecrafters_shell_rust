use std::{
    env,
    fs::read_dir,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    str::FromStr,
};

pub struct PathCollection {
    paths: Vec<String>,
}

impl FromStr for PathCollection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PathCollection {
            paths: s.split(":").map(|s| s.to_owned()).collect(),
        })
    }
}

impl PathCollection {
    pub fn build() -> Result<Self, &'static str> {
        let Ok(path) = env::var("PATH") else {
            return Err("Path not set");
        };
        PathCollection::from_str(&path)
    }

    // fn recurse(path: impl AsRef<Path>) -> Vec<PathBuf> {
    // let Ok(entries) = read_dir(path) else { return vec![] };
    // entries.flatten().flat_map(|entry| {
    //     let Ok(meta) = entry.metadata() else { return vec![] };
    //     if meta.is_dir() { return recurse(entry.path()); }
    //     if meta.is_file() { return vec![entry.path()]; }
    //     vec![]
    // }).collect()

    pub fn list_files(path: &Path) -> Vec<String> {
        if let Ok(entries) = read_dir(path) {
            let res: Vec<String> = entries
                .filter_map(|entry| {
                    if let Ok(file) = entry {
                        if let Ok(data) = file.metadata() {
                            if data.is_file() {
                                Some(file.file_name().into_string().unwrap())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            res
        } else {
            Vec::new()
        }
    }

    pub fn list(&self) -> Vec<String> {
        let mut files = Vec::new();
        for path in &self.paths {
            let path = Path::new(&path);
            files.extend(PathCollection::list_files(path));
        }
        files
    }

    pub fn find(&self, cmd: String) -> Option<String> {
        for path in &self.paths {
            let path = Path::new(&path).join(&cmd);
            if !path.exists() {
                continue;
            }
            let Ok(meta) = path.metadata() else {
                continue;
            };

            if meta.permissions().mode() & 0o111 != 0 {
                return path.to_str().map(|s| s.to_string());
            }
        }
        None
    }
}
