use std::{path::PathBuf, fs};
use dirs;

static LINUX_APP_DIRECTORY: &'static str = ".consul-desktop-ui";

struct LinuxFileSystemAppHelper {}

impl LinuxFileSystemAppHelper {
    pub fn new() -> LinuxFileSystemAppHelper {
        LinuxFileSystemAppHelper {}
    }
}

pub trait FileSystemAppHelper {
    fn initialize_app_directory(&self) -> Result<(), std::io::Error>;
    fn get_path(&self, relative_path: &PathBuf) -> PathBuf;
}


impl FileSystemAppHelper for LinuxFileSystemAppHelper {
    fn initialize_app_directory(&self) -> Result<(), std::io::Error> {
        let mut path = dirs::home_dir().unwrap();
        path.push(LINUX_APP_DIRECTORY);

        fs::create_dir_all(path)?;

        Ok(())
    }
    fn get_path(&self, relative_path: &PathBuf) -> PathBuf {
        let mut path = dirs::home_dir().unwrap();
        path.push(LINUX_APP_DIRECTORY);
        path.push(relative_path);

        path
    }
}

pub fn get_file_system_helper() -> Box<dyn FileSystemAppHelper> {
    Box::new(LinuxFileSystemAppHelper::new())
}