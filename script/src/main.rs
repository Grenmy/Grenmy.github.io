//! # Markdown to html
//!
//! This program converts a directory of markdown files to html files.
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
//!
//! ## Idee
//! - usare axum per creare un server che serve i file html (molto simile a actix-web)

mod md_to_html;
use chrono::Local;
use md_to_html::md_to_html;

fn main() -> Result<(), std::io::Error> {
    let target = "../";
    let dest = "../";
    match md_to_html(&target, &dest) {
        // print DD/MM/YYYY-HH:MM:SS
        Ok(_) => println!("success {}", Local::now().format("%d/%m/%Y-%H:%M:%S")),
        Err(e) => println!("error: {:?}", e),
    }

    Ok(())
}
