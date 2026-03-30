use clap::Parser;
use std::fs;
use std::path::PathBuf;
use toml_edit::DocumentMut;

const VARIABLES_CSS: &str = include_str!("../gruvbox/css/variables.css");
const HIGHLIGHT_CSS: &str = include_str!("../gruvbox/highlight.css");
const GRUVBOX_JS: &str = include_str!("../gruvbox/gruvbox.js");

#[derive(Parser)]
#[command(name = "mdbook-gruvbox", about = "Gruvbox dark theme for mdBook")]
enum Cli {
    /// Install the Gruvbox theme into an mdBook project
    Install {
        /// Path to the mdBook project root (default: current directory)
        path: Option<PathBuf>,
    },
}

fn main() {
    let Cli::Install { path } = Cli::parse();
    let root = path.unwrap_or_else(|| PathBuf::from("."));

    if let Err(e) = install(&root) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn install(root: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let book_toml_path = root.join("book.toml");
    if !book_toml_path.exists() {
        return Err(format!("book.toml not found at {}", book_toml_path.display()).into());
    }

    // Write theme files
    let gruvbox_dir = root.join("gruvbox");
    let css_dir = gruvbox_dir.join("css");
    fs::create_dir_all(&css_dir)?;

    let files = [
        (css_dir.join("variables.css"), VARIABLES_CSS),
        (gruvbox_dir.join("highlight.css"), HIGHLIGHT_CSS),
        (gruvbox_dir.join("gruvbox.js"), GRUVBOX_JS),
    ];

    for (path, content) in &files {
        fs::write(path, content)?;
        println!("  wrote {}", path.display());
    }

    // Clean up stale files from old installation approach
    let stale_files = [
        gruvbox_dir.join("index.hbs"),
        gruvbox_dir.join("ayu-highlight.css"),
        gruvbox_dir.join("tomorrow-night.css"),
    ];
    for path in &stale_files {
        if path.exists() {
            fs::remove_file(path)?;
            println!("  removed stale {}", path.display());
        }
    }

    // Update book.toml
    let toml_str = fs::read_to_string(&book_toml_path)?;
    let mut doc = toml_str.parse::<DocumentMut>()?;

    if !doc.contains_table("output") {
        doc["output"] = toml_edit::Item::Table(toml_edit::Table::new());
    }
    if !doc["output"].as_table().unwrap().contains_key("html") {
        doc["output"]["html"] = toml_edit::Item::Table(toml_edit::Table::new());
    }

    // Remove stale theme directory override from old installs
    if let Some(html) = doc["output"]["html"].as_table_mut() {
        html.remove("theme");
    }

    doc["output"]["html"]["default-theme"] = toml_edit::value("gruvbox");
    doc["output"]["html"]["preferred-dark-theme"] = toml_edit::value("gruvbox");

    // Add CSS and JS via additional-css / additional-js arrays
    ensure_in_array(&mut doc, "additional-css", &[
        "gruvbox/css/variables.css",
        "gruvbox/highlight.css",
    ]);
    ensure_in_array(&mut doc, "additional-js", &[
        "gruvbox/gruvbox.js",
    ]);

    fs::write(&book_toml_path, doc.to_string())?;
    println!("  updated {}", book_toml_path.display());

    println!("\ngruvbox theme installed! run `mdbook build` to build your book.");
    Ok(())
}

fn ensure_in_array(doc: &mut DocumentMut, key: &str, values: &[&str]) {
    let html = doc["output"]["html"].as_table_mut().unwrap();

    if !html.contains_key(key) {
        let mut arr = toml_edit::Array::new();
        for v in values {
            arr.push(v.to_string());
        }
        html[key] = toml_edit::value(arr);
    } else {
        let arr = html[key]
            .as_value_mut()
            .and_then(|v| v.as_array_mut())
            .expect(&format!("{key} should be an array"));

        for v in values {
            let already_present = arr.iter().any(|existing| {
                existing.as_str().map_or(false, |s| s == *v)
            });
            if !already_present {
                arr.push(v.to_string());
            }
        }
    }
}
