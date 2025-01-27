//! # Markdown to HTML
//!
//! This program converts a directory of Markdown files to HTML files.
//! The structure of the directory needs to be the following:
//! ```markdown
//! target
//! ├── folder1
//! │   ├── file1.md
//! │   ├── file2.md
//! │   └── ...
//! ├── layout
//! │   ├── some_layout.md
//! │   └── ...
//! ├── assets
//! │   ├── some_asset.md
//! │   └── ...
//! └── ...
//! ```
//!
//! Once the program is run, it will wait for an event to occur in the target
//! directory.

mod md_to_html;
use md_to_html::md_to_html;

fn main() -> Result<(), std::io::Error> {
    let src = "path/to/md";
    let out = "path/to/out";

    md_to_html(&src, &out)
}
