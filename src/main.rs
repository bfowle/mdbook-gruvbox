use clap::Parser;
use std::fs;
use std::path::PathBuf;
use toml_edit::DocumentMut;

const VARIABLES_CSS: &str = include_str!("../gruvbox/css/variables.css");
const HIGHLIGHT_CSS: &str = include_str!("../gruvbox/highlight.css");
const AYU_HIGHLIGHT_CSS: &str = include_str!("../gruvbox/ayu-highlight.css");
const TOMORROW_NIGHT_CSS: &str = include_str!("../gruvbox/tomorrow-night.css");

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
        (gruvbox_dir.join("ayu-highlight.css"), AYU_HIGHLIGHT_CSS),
        (gruvbox_dir.join("tomorrow-night.css"), TOMORROW_NIGHT_CSS),
    ];

    for (path, content) in &files {
        fs::write(path, content)?;
        println!("  wrote {}", path.display());
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
    doc["output"]["html"]["theme"] = toml_edit::value("gruvbox");

    fs::write(&book_toml_path, doc.to_string())?;
    println!("  updated {}", book_toml_path.display());

    println!("\ngruvbox theme installed! run `mdbook build` to build your book.");
    Ok(())
}
