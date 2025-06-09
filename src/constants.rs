/// this path is used to for accessing other workspace norg files
pub const WORKSPACE_PATH: &'static str = "/view/workspace";

/// this path is used to access files from current workspace norg files
pub const CURRENT_WORKSPACE_PATH: &'static str = "/view/current";

/// this path is used to access files from root directory
pub const SYSTEM_PATH: &'static str = "/view/root";

/// this path is used to access files from home directory
pub const HOME_PATH: &'static str = "/view/home";

pub const ARG_RAW: &'static str = "raw";

pub const ARG_RAW_POSSIBLE_VALS: [&'static str; 3] = ["1", "true", "yes"];

pub mod paths {
    use super::*;
    use const_format::concatcp;

    /// path for rendering root of current workspace
    pub const CURRENT_WORKSPACE_ROOT: &'static str =
        concatcp!(CURRENT_WORKSPACE_PATH, "/index.norg");
    /// path for rendering any file from current workspace
    pub const CURRENT_WORKSPACE_FILE: &'static str =
        concatcp!(CURRENT_WORKSPACE_PATH, "/*file_path");
    /// load files from root of the file system
    pub const SYSTEM_FILES: &'static str = concatcp!(SYSTEM_PATH, "/*file_path");
    /// load files from root of the file system
    pub const HOME_FILES: &'static str = concatcp!(HOME_PATH, "/*file_path");

    pub const DIRECTORY_SERVE: &'static str = "/view/fs";
}
