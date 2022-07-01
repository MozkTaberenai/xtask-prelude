use super::*;

pub fn current_dir() -> Result<PathBuf> {
    let path = std::env::current_dir()?;
    info!("current_dir() -> {path:?}");
    Ok(path)
}

pub fn set_current_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    info!("set_current_dir(path: {path:?})");
    std::env::set_current_dir(path)?;
    Ok(())
}

pub fn var(key: impl AsRef<OsStr>) -> Result<String> {
    let key = key.as_ref();
    let value = std::env::var(key)?;
    info!("var(key: {key:?}) -> value: {value:?}");
    Ok(value)
}

// #[instrument(skip(key, value))]
pub fn set_var(key: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Result<()> {
    let key = key.as_ref();
    let value = value.as_ref();
    info!("set_var(key: {key:?}, value: {value:?})");
    std::env::set_var(key, value);
    Ok(())
}
