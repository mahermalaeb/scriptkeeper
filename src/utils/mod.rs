pub mod short_temp_files;

use crate::R;
use std::fs;
use std::path::Path;

pub fn path_to_string(path: &Path) -> R<&str> {
    Ok(path
        .to_str()
        .ok_or_else(|| format!("invalid utf8 sequence: {:?}", &path))?)
}

pub fn parse_shebang(program: &Path) -> Option<String> {
    let contents = fs::read(program).ok()?;
    if contents.starts_with(b"#!") {
        let bytes = contents
            .into_iter()
            .take_while(|&byte| byte != b'\n')
            .collect::<Vec<_>>();
        Some(String::from_utf8_lossy(&bytes).to_string())
    } else {
        None
    }
}
