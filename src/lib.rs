use std::{env::set_current_dir, fs::File, io::{BufReader, BufRead, BufWriter, Write}};

use toml::Value;
use cnctd_shell::Shell;

pub struct RustCrate {
    pub name: String,
    pub features: Option<Vec<String>>
}

impl RustCrate {
    pub fn new(name: &str, features: Option<Vec<&str>>) -> Self {
        Self { 
            name: name.to_string(), 
            features: features.map(|vec| vec.iter().map(|&s| s.to_string()).collect()),
        }
    }
}

pub enum Crate {
    Warp,
    Tokio,
    Dotenv,
    Reqwest,
    State,
    LocalIpAddress,
    Serde,
    SerdeJson,
    Chrono,
    Diesel,
    RusotoSecretsmanager,
    RusotoCore,
    Uuid,
    Anyhow,
    ChronoTz,
    TokioStream,
    Futures,
    Imap,
    NativeTls,
    Regex,
    Mailparse,
    Csv,
    Redis,
}

impl Crate {
    pub fn to_rust_crate(&self) -> RustCrate {
        match self {
            Crate::Warp => RustCrate::new("warp", None),
            Crate::Tokio => RustCrate::new("tokio", Some(vec!["full"])),
            Crate::Dotenv => RustCrate::new("dotenv", None),
            Crate::Reqwest => RustCrate::new("reqwest", Some(vec!["json"])),
            Crate::State => RustCrate::new("state", None),
            Crate::LocalIpAddress => RustCrate::new("local-ip-address", None),
            Crate::Serde => RustCrate::new("serde", Some(vec!["derive", "rc"])),
            Crate::SerdeJson => RustCrate::new("serde_json", None),
            Crate::Chrono => RustCrate::new("chrono", None),
            Crate::Diesel => RustCrate::new("diesel", Some(vec!["postgres", "chrono", "serde_json"])),
            Crate::RusotoSecretsmanager => RustCrate::new("rusoto_secretsmanager", None),
            Crate::RusotoCore => RustCrate::new("rusoto_core", None),
            Crate::Uuid => RustCrate::new("uuid", Some(vec!["v4"])),
            Crate::Anyhow => RustCrate::new("anyhow", None),
            Crate::ChronoTz => RustCrate::new("chrono-tz", None),
            Crate::TokioStream => RustCrate::new("tokio-stream", None),
            Crate::Futures => RustCrate::new("futures", None),
            Crate::Imap => RustCrate::new("imap", None),
            Crate::NativeTls => RustCrate::new("native-tls", None),
            Crate::Regex => RustCrate::new("regex", None),
            Crate::Mailparse => RustCrate::new("mailparse", None),
            Crate::Csv => RustCrate::new("csv", None),
            Crate::Redis => RustCrate::new("redis", Some(vec!["tokio-comp"])),
        }
    }

}


pub struct Cargo {}

impl Cargo {
    pub async fn bump_version(version_part: &str) -> anyhow::Result<()> {
        let file = File::open("Cargo.toml")?;
        let reader = BufReader::new(file);
    
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            let mut line = line?;
            if line.starts_with("version = ") {
                let version = line.split_off(10).trim_matches('"').to_string();
                let mut parts: Vec<i64> = version.split('.').map(|s| s.parse().unwrap()).collect();
                
                match version_part {
                    "major" => {
                        parts[0] += 1;
                        parts[1] = 0;
                        parts[2] = 0;
                    },
                    "minor" => {
                        parts[1] += 1;
                        parts[2] = 0;
                    },
                    "patch" => {
                        parts[2] += 1;
                    },
                    _ => return Err(anyhow::anyhow!("Invalid version part")),
                }
    
                let new_version = format!("version = \"{}.{}.{}\"", parts[0], parts[1], parts[2]);
                lines.push(new_version);
            } else {
                lines.push(line);
            }
        }
    
        let file = File::create("Cargo.toml")?;
        let mut writer = BufWriter::new(file);
        for line in lines {
            writeln!(writer, "{}", line)?;
        }
    
        Ok(())
    }
    
    
    pub async fn install_app() -> Result<(), anyhow::Error> {
        Shell::run("cargo install --path .", true).await?;
        Ok(())
    }
    
    pub async fn init(path: &str) -> anyhow::Result<()> {
        set_current_dir(path)?;
        let command = format!("cargo init");
        Shell::run(&command, false).await?;
        Ok(())
    }
    
    pub fn get_app_version(package_dir: &str) -> Result<String, Box<dyn std::error::Error>> {
        let toml_str = std::fs::read_to_string(format!("{}/Cargo.toml", package_dir))?;
        let parsed_toml: Value = toml::from_str(&toml_str)?;
    
        let version = parsed_toml
            .get("package")
            .and_then(|package| package.get("version"))
            .and_then(|version| version.as_str())
            .ok_or_else(|| "could not parse version from Cargo.toml")?;
    
        Ok(version.to_owned())
    }
    
    pub async fn install_crate(crate_name: &str) -> anyhow::Result<()> {
        let command = format!("cargo add {}", crate_name);
        Shell::run(&command, true).await?;
        Ok(())
    }
    
    pub async fn check_for_rust_and_cargo() -> Result<(), anyhow::Error> {
        let rust_status = Shell::run_with_exit_status("rustc --version", false).await?;
        let cargo_status = Shell::run_with_exit_status("cargo --version", false).await?;
    
        // println!("rust status: {}", rust_status);
        // println!("cargo status: {}", cargo_status);
        if rust_status == 0 && cargo_status == 0 {
            // println!("Rust and Cargo are installed.");
        } else {
            if rust_status != 0 {
                println!("Rust is not installed.");
            }
            if cargo_status != 0 {
                println!("Cargo is not installed.");
            }
        }
    
        Ok(())
    }
}


