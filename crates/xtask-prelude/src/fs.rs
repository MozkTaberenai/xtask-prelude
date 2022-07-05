use super::*;

pub fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    info!("rename from={from:?} to={to:?}");
    std::fs::rename(from, to)?;
    Ok(())
}

pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();
    info!("copy from={from:?} to={to:?}");
    std::fs::copy(from, to)?;
    Ok(())
}

pub fn create_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    info!("create_dir path={path:?}");
    std::fs::create_dir(path)?;
    Ok(())
}

pub fn create_dir_all(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    info!("create_dir_all path={path:?}");
    std::fs::create_dir_all(path)?;
    Ok(())
}

pub fn remove_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    info!("remove_dir path={path:?}");
    std::fs::remove_dir(path)?;
    Ok(())
}

pub fn remove_dir_all(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    info!("remove_dir_all path={path:?}");
    std::fs::remove_dir_all(path)?;
    Ok(())
}

pub fn remove_file(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    info!("remove_file path={path:?}");
    std::fs::remove_file(path)?;
    Ok(())
}

pub fn read_dir(path: impl AsRef<Path>) -> Result<std::fs::ReadDir> {
    let path = path.as_ref();
    info!("read_dir path={path:?}");
    let r = std::fs::read_dir(path)?;
    Ok(r)
}

pub fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let path = path.as_ref();
    info!("read path={path:?}");
    let r = std::fs::read(path)?;
    Ok(r)
}

pub fn read_to_string(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    info!("read_to_string path={path:?}");
    let r = std::fs::read_to_string(path)?;
    Ok(r)
}

pub fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> Result<()> {
    let path = path.as_ref();
    let contents = contents.as_ref();
    info!(
        "write path={path:?} contents={contents_bytes} bytes",
        contents_bytes = contents.len()
    );
    std::fs::write(path, contents)?;
    Ok(())
}
