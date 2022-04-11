use std::path::Path;

use crate::config::CONFIG_FILE;

pub fn is_workspace(path: &Path) -> bool {
    for parent in path.ancestors() {
        if parent.join(CONFIG_FILE).exists() {
            return true;
        }
    }

    false
}
