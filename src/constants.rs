/// this path is used to for accessing other workspace norg files
pub const WORKSPACE_PATH: &str = "/view/workspace";

/// this path is used to access files from current workspace norg files
pub const CURRENT_WORKSPACE_PATH: &str = "/view/current";

/// this path is used to access files from root directory
pub const SYSTEM_PATH: &str = "/view/root";

/// this path is used to access files from home directory
pub const HOME_PATH: &str = "/view/home";

pub const ARG_RAW: &str = "raw";

pub const ARG_RAW_POSSIBLE_VALS: [&str; 3] = ["1", "true", "yes"];

pub mod paths {
    use super::*;
    use const_format::concatcp;

    /// path for rendering root of current workspace
    pub const CURRENT_WORKSPACE_ROOT: &str =
        concatcp!(CURRENT_WORKSPACE_PATH, "/index.norg");
    /// path for rendering any file from current workspace
    pub const CURRENT_WORKSPACE_FILE: &str =
        concatcp!(CURRENT_WORKSPACE_PATH, "/*file_path");
    /// load files from root of the file system
    pub const SYSTEM_FILES: &str = concatcp!(SYSTEM_PATH, "/*file_path");
    /// load files from root of the file system
    pub const HOME_FILES: &str = concatcp!(HOME_PATH, "/*file_path");

    pub const DIRECTORY_SERVE: &str = "/view/fs";
}
