use clap::{Parser, Subcommand};
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Simple CLI URL shortener (in-memory only).
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Shorten a URL
    Shorten { url: String },
    /// Resolve a short code
    Resolve { code: String },
}

/// A very basic in-memory database for shortened links
struct UrlStore {
    map: Mutex<HashMap<String, String>>,
}

impl UrlStore {
    fn new() -> Self {
        UrlStore {
            map: Mutex::new(HashMap::new()),
        }
    }

    fn shorten(&self, url: String) -> String {
        let code: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        let mut m = self.map.lock().unwrap();
        m.insert(code.clone(), url);
        code
    }

    fn resolve(&self, code: &str) -> Option<String> {
        let m = self.map.lock().unwrap();
        m.get(code).cloned()
    }
}

fn main() {
    let cli = Cli::parse();
    let store = Arc::new(UrlStore::new());

    match cli.command {
        Commands::Shorten { url } => {
            let code = store.shorten(url);
            println!("Short code: {}", code);
        }
        Commands::Resolve { code } => match store.resolve(&code) {
            Some(url) => println!("Original URL: {}", url),
            None => println!("Code not found"),
        },
    }
}
