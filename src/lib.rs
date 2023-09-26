use std::{env::set_current_dir, fs::{File, create_dir_all}, io::{BufReader, BufRead, BufWriter, Write}};
use cargo_toml::Author;
use serde::{Deserialize, Serialize};
use cnctd_rest::Rest;
use cnctd_shell::Shell;
use toml::Value;
use toml_edit::{Document, value};

pub mod cargo_toml;

#[derive(Debug, Deserialize, Serialize)]
pub enum CrateType {
    App,
    Module
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RustCrate {
    pub name: String,
    pub features: Option<Vec<String>>,
    pub crate_type: CrateType,
}

impl RustCrate {
    pub fn new(name: &str, features: Option<Vec<&str>>, crate_type: CrateType) -> Self {
        Self { 
            name: name.to_string(), 
            features: features.map(|vec| vec.iter().map(|&s| s.to_string()).collect()),
            crate_type
        }
    }
}



#[derive(Debug, Deserialize, Serialize)]
pub struct RustModule {

}

#[derive(Debug, Deserialize, Serialize)]
pub struct RustApp {

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
            Crate::Warp => RustCrate::new("warp", None, CrateType::Module),
            Crate::Tokio => RustCrate::new("tokio", Some(vec!["full"]), CrateType::Module),
            Crate::Dotenv => RustCrate::new("dotenv", None, CrateType::Module),
            Crate::Reqwest => RustCrate::new("reqwest", Some(vec!["json"]), CrateType::Module),
            Crate::State => RustCrate::new("state", None, CrateType::Module),
            Crate::LocalIpAddress => RustCrate::new("local-ip-address", None, CrateType::Module),
            Crate::Serde => RustCrate::new("serde", Some(vec!["derive", "rc"]), CrateType::Module),
            Crate::SerdeJson => RustCrate::new("serde_json", None, CrateType::Module),
            Crate::Chrono => RustCrate::new("chrono", None, CrateType::Module),
            Crate::Diesel => RustCrate::new("diesel", Some(vec!["postgres", "chrono", "serde_json"]), CrateType::Module),
            Crate::RusotoSecretsmanager => RustCrate::new("rusoto_secretsmanager", None, CrateType::Module),
            Crate::RusotoCore => RustCrate::new("rusoto_core", None, CrateType::Module),
            Crate::Uuid => RustCrate::new("uuid", Some(vec!["v4"]), CrateType::Module),
            Crate::Anyhow => RustCrate::new("anyhow", None, CrateType::Module),
            Crate::ChronoTz => RustCrate::new("chrono-tz", None, CrateType::Module),
            Crate::TokioStream => RustCrate::new("tokio-stream", None, CrateType::Module),
            Crate::Futures => RustCrate::new("futures", None, CrateType::Module),
            Crate::Imap => RustCrate::new("imap", None, CrateType::Module),
            Crate::NativeTls => RustCrate::new("native-tls", None, CrateType::Module),
            Crate::Regex => RustCrate::new("regex", None, CrateType::Module),
            Crate::Mailparse => RustCrate::new("mailparse", None, CrateType::Module),
            Crate::Csv => RustCrate::new("csv", None, CrateType::Module),
            Crate::Redis => RustCrate::new("redis", Some(vec!["tokio-comp"]), CrateType::Module),
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
    
    pub async fn init(path: &str, crate_type: CrateType) -> anyhow::Result<()> {
        create_dir_all(&path)?;
        set_current_dir(path)?;
        let command = match crate_type {
            CrateType::App => "cargo init",
            CrateType::Module => "cargo init --lib"
        };
        
        Shell::run(command, false).await?;
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
    
        if rust_status == 0 && cargo_status == 0 {
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
    
    pub async fn get_latest_crate_version(crate_name: &str) -> anyhow::Result<String> {
        let url = format!("https://crates.io/api/v1/crates/{}", crate_name);
        let json: Value = Rest::get(&url).await?;
        Ok(json["crate"]["max_version"].as_str().unwrap().to_string())
    }

    pub async fn update_cargo_toml(author: Author, description: &str, repository: &str, license: &str, crate_type: CrateType) -> anyhow::Result<()> {
        // Read the existing Cargo.toml into a string
        let data = std::fs::read_to_string("Cargo.toml")?;
        
        // Parse the string into a TOML Document
        let mut doc = data.parse::<Document>()?;
        
        // Access the [package] table
        let package = doc["package"].as_table_mut().unwrap();
        
        // Create an Item for authors
        let mut authors_array = toml_edit::Array::new();
        let author_str = format!("{} <{}>", author.name, author.email);
        authors_array.push(author.organization);
        authors_array.push(author_str);
        

        let mut keywords_array = toml_edit::Array::new();
        match crate_type {
            CrateType::App => keywords_array.push("app"),
            CrateType::Module => keywords_array.push("module"),
        }
        
        // Update fields
        package.insert("authors", value(authors_array));
        package.insert("description", value(description));
        package.insert("repository", value(repository));
        package.insert("license", value(license));
        package.insert("keywords", value(keywords_array));
        
        // Write the Document back to Cargo.toml
        std::fs::write("Cargo.toml", doc.to_string())?;
        
        Ok(())
    }
    
    pub async fn publish_crate(path: &str) -> anyhow::Result<()> {
        set_current_dir(path)?;
        Shell::run("cargo publish", false).await?;

        Ok(())
    }
    
    // pub fn update_rust_project_versions(root_path: &str) -> std::io::Result<()> {
    //     let mut project_versions: HashMap<String, String> = HashMap::new();
        
    //     for entry in WalkDir::new(root_path)
    //         .into_iter()
    //         .filter_entry(|e| !is_ignored(e))
    //     {
    //         let entry = entry?;
    //         let path = entry.path();
    //         let file_name = path.file_name().unwrap_or_default();
    
    //         if path.is_file() && file_name.to_str().unwrap() == "Cargo.toml" {
    //             let contents = std::fs::read_to_string(path)?;
    //             let toml: TomlValue = contents.parse().unwrap();
                
    //             if let Some(package) = toml.get("package") {
    //                 if let Some(name) = package.get("name").and_then(TomlValue::as_str) {
    //                     if let Some(version) = package.get("version").and_then(TomlValue::as_str) {
    //                         project_versions.insert(name.to_string(), version.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    
    //     for entry in WalkDir::new(root_path)
    //         .into_iter()
    //         .filter_entry(|e| !is_ignored(e))
    //     {
    //         let entry = entry?;
    //         let path = entry.path();
    //         let file_name = path.file_name().unwrap_or_default();
    
    //         if path.is_file() && file_name.to_str().unwrap() == "Cargo.toml" {
    //             let mut contents = std::fs::read_to_string(path)?;
    //             let mut doc = contents.parse::<Document>().unwrap();
    
    //             if let Some(table) = doc.as_table_mut().entry("dependencies").as_table_mut() {
    //                 for (name, version) in project_versions.iter() {
    //                     if let Some(dep) = table.get(name) {
    //                         if let Item::Table(dep_table) = dep {
    //                             if dep_table.contains_key("version") && dep_table.contains_key("path") {
    //                                 dep_table.get_mut("version").unwrap().as_value_mut().unwrap().as_str_mut().unwrap().replace_range(.., version);
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    
    //             std::fs::write(path, doc.to_string_in_original_order())?;
    //         }
    //     }
    
    //     Ok(())
    // }
}

// fn is_ignored(entry: &DirEntry) -> bool {
//     entry.file_name().to_str().map(|s| s == "target" || s == "node_modules").unwrap_or(false)
// }