mod constants;

pub use constants::{bsearch_icon_table, EXACTMATCH_ICON_TABLE, EXTENSION_ICON_TABLE};

use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

pub const DEFAULT_ICON: char = '';
pub const FOLDER_ICON: char = '';
pub const DEFAULT_FILER_ICON: char = '';

/// The type used to represent icons.
///
/// This could be changed into different type later,
/// so functions take and return this type, not `char` or `&str` directly.
type Icon = char;

/// Return appropriate icon for the path. If no icon matched, return the specified default one.
///
/// Try matching the exactmatch map against the file name, and then the extension map.
#[inline]
pub fn get_icon_or(path: &Path, default: Icon) -> Icon {
    path.file_name()
        .and_then(std::ffi::OsStr::to_str)
        .and_then(|filename| {
            bsearch_icon_table(&filename.to_lowercase().as_str(), EXACTMATCH_ICON_TABLE)
                .map(|idx| EXACTMATCH_ICON_TABLE[idx].1)
        })
        .unwrap_or_else(|| {
            path.extension()
                .and_then(std::ffi::OsStr::to_str)
                .and_then(|ext| {
                    bsearch_icon_table(ext, EXTENSION_ICON_TABLE)
                        .map(|idx| EXTENSION_ICON_TABLE[idx].1)
                })
                .unwrap_or(default)
        })
}

fn icon_for(line: &str) -> Icon {
    let path = Path::new(line);
    get_icon_or(&path, DEFAULT_ICON)
}

pub fn prepend_icon(line: &str) -> String {
    format!("{} {}", icon_for(line), line)
}

#[inline]
pub fn icon_for_filer(path: &Path) -> Icon {
    if path.is_dir() {
        FOLDER_ICON
    } else {
        get_icon_or(path, DEFAULT_FILER_ICON)
    }
}

pub fn prepend_filer_icon(path: &Path, line: &str) -> String {
    format!("{} {}", icon_for_filer(path), line)
}

pub fn prepend_grep_icon(line: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.*):\d+:\d+:").unwrap();
    }
    let icon = RE
        .captures(line)
        .and_then(|cap| cap.get(1))
        .map(|m| icon_for(m.as_str()))
        .unwrap_or(DEFAULT_ICON);
    format!("{} {}", icon, line)
}
