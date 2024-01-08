mod template;

use clap::{Parser, Subcommand};
use include_dir::{include_dir, Dir, DirEntry};
use std::collections::HashMap;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use template::template;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    New { name: String },
}

pub fn extract<S: AsRef<Path>>(
    dir: &'_ Dir<'_>,
    base_path: S,
    vars: &HashMap<String, String>,
) -> std::io::Result<()> {
    let base_path = base_path.as_ref();

    for entry in dir.entries() {
        let path = base_path.join(entry.path());

        match entry {
            DirEntry::Dir(d) => {
                std::fs::create_dir_all(&path)?;
                extract(d, base_path, vars)?;
            }
            DirEntry::File(f) => {
                let Some(ext) = path.extension() else {
                    continue;
                };
                if ext.to_string_lossy() != "template" {
                    continue;
                }
                let path = PathBuf::from(path.to_string_lossy().strip_suffix(".template").unwrap());
                let content = template(
                    std::str::from_utf8(f.contents()).expect("UTF8 template"),
                    vars,
                )
                .unwrap();
                std::fs::write(path, content.as_bytes())?
            }
        }
    }

    Ok(())
}

static TEMPLATES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Command::New { name } => {
            let mut vars = HashMap::new();
            vars.insert("name".to_string(), name.clone());
            create_dir(&name)?;
            extract(&TEMPLATES_DIR, &name, &vars)?;
            Ok(())
        }
    }
}
