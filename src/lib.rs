use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    file_path: Option<PathBuf>,
    /// lines count only
    #[arg(short, long)]
    lines: bool,
    /// words count only
    #[arg(short, long)]
    words: bool,
    /// characters count only
    #[arg(short, long)]
    chars: bool,
}

#[derive(Debug, PartialEq)]
struct Counter {
    lines: usize,
    words: usize,
    chars: usize,
}

fn counts(text: &str) -> Counter {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();

    Counter {
        lines,
        words,
        chars,
    }
}

pub fn run(cli: Cli) -> Result<()> {
    let text_content = match &cli.file_path {
        Some(path) => fs::read_to_string(path)
            .with_context(|| format!("Can't read file {}", path.display()))?,
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("Can't read from stdin")?;
            buf
        }
    };
    let content_count = counts(&text_content);

    let show_all = !cli.lines && !cli.words && !cli.chars;
    let mut parts = Vec::new();
    if cli.lines || show_all {
        parts.push(format!("lines: {}", content_count.lines));
    }

    if cli.words || show_all {
        parts.push(format!("words: {}", content_count.words));
    }

    if cli.chars || show_all {
        parts.push(format!("chars: {}", content_count.chars));
    }

    println!("{}", parts.join(", "));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_count() {
        let text = "";
        assert_eq!(
            counts(text),
            Counter {
                lines: 0,
                words: 0,
                chars: 0
            }
        );
    }

    #[test]
    fn single_line_count() {
        let text = "hello world";
        assert_eq!(
            counts(text),
            Counter {
                lines: 1,
                words: 2,
                chars: 11
            }
        );
    }

    #[test]
    fn multi_line_count() {
        let text = "a\nb\nc";
        assert_eq!(
            counts(text),
            Counter {
                lines: 3,
                words: 3,
                chars: 5
            }
        );
    }
}
