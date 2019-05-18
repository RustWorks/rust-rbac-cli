use std::fs::*;
use std::path::*;
use std::io::*;

pub fn list_files(path: &Path) -> Vec<String> {
    read_dir(path)
        .unwrap()
        .map(|p| p.unwrap().path().display().to_string())
        .collect::<Vec<String>>()
}

pub fn read_file(path: &Path) -> Result<String> {
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    Ok(content)
}

pub fn write_to_file(path: &Path, content: &str) -> Result<()> {
    OpenOptions::new().create(false).write(true).append(true)
        .open(path)
        .and_then(|ref mut f| writeln!(f, "{}", content))
}
