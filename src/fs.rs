use std::{env, os::unix::fs::PermissionsExt, path::{Path}, str::FromStr};

pub struct PathCollection{
    paths : Vec<String>
}

impl FromStr for PathCollection{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       Ok(
        PathCollection{
         paths : s.split(":").map(|s| s.to_owned()).collect()
        })
    }
}

impl PathCollection {
    pub fn build() -> Result<Self,&'static str> {
        let Ok(path) = env::var("PATH") else {
            return Err("Path not set");
        };
        PathCollection::from_str(&path)
    }

    pub fn find(&self, cmd: String) -> Option<String> {
        for path in &self.paths {
            let path = Path::new(&path).join(&cmd);
            if !path.exists() {
                continue;
            }
            let Ok(meta) = path.metadata() else { continue;};

            if meta.permissions().mode() & 0o111 != 0 {
                return path.to_str().map(|s| s.to_string());               
            }            
        }
        None
    }
}
