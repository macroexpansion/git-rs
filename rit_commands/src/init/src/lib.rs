pub fn init() -> std::io::Result<()> {
    let path = "./.rit";
    create_dot_folder(path)?;
    create_hooks_folder(path)?;
    create_info_folder(path)?;
    create_logs_folder(path)?;
    create_objects_folder(path)?;
    create_refs_folder(path)?;
    Ok(())
}

fn create_dot_folder(path: &str) -> std::io::Result<()> {
    std::fs::create_dir(path)?;
    Ok(())
}

fn create_hooks_folder(path: &str) -> std::io::Result<()> {
    std::fs::create_dir(format!("{}/hooks", path))?;
    Ok(())
}

fn create_info_folder(path: &str) -> std::io::Result<()> {
    std::fs::create_dir(format!("{}/info", path))?;
    Ok(())
}

fn create_logs_folder(path: &str) -> std::io::Result<()> {
    std::fs::create_dir(format!("{}/logs", path))?;
    Ok(())
}

fn create_objects_folder(path: &str) -> std::io::Result<()> {
    std::fs::create_dir(format!("{}/objects", path))?;
    Ok(())
}

fn create_refs_folder(path: &str) -> std::io::Result<()> {
    std::fs::create_dir(format!("{}/refs", path))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /* #[test]
    fn create_and_remove_dot_folder() {
        let path = "./.rit";
        assert!(create_dot_folder(path).is_ok());
        std::fs::remove_dir(path).unwrap();
    } */

    #[test]
    fn test_init_command() {
        assert!(init().is_ok());
    }
}
