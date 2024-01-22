use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};
use config::Environment;
use dotenv::dotenv;
use tracing::log;


#[derive(Debug, Deserialize)]
pub struct Configs {
    pub server: Server,
    pub database: DataBase,
    pub cert: Cert,
    pub jwt: Jwt,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub address: String,
    #[serde(deserialize_with = "deserialize_cors_allow_origin")]
    pub cors_allow_origin: Vec<String>,
    pub ssl: bool,
}

#[derive(Debug, Deserialize)]
pub struct DataBase {
    pub database_url: String,
}

#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub jwt_secret: String,
    pub jwt_exp: i64,
}

#[derive(Debug, Deserialize)]
pub struct Cert {
    /// cert
    pub cert: String,
    /// key
    pub key: String,
}

const CONFIG_FILE: &str = "config/config.yml";

// pub static CFG: Lazy<Configs> = Lazy::new(self::Configs::init_by_serde_yaml);
pub static CFG: Lazy<Configs> = Lazy::new(self::Configs::init_by_config_and_environment);

fn deserialize_cors_allow_origin<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    // Deserialize the value as a single string
    let cors_string: String = Deserialize::deserialize(deserializer)?;

    // Split the string by commas and collect into a Vec<String>
    let cors_vec: Vec<String> = cors_string.split(',').map(String::from).collect();

    Ok(cors_vec)
}

impl Configs {
    ///by serde_yaml get the config
    pub fn init_by_serde_yaml() -> Self {
        let mut file = match File::open(CONFIG_FILE) {
            Ok(f) => f,
            Err(e) => panic!(
                "Configuration file does not exist:{},error message:{}",
                CONFIG_FILE, e
            ),
        };
        let mut cfg_contents = String::new();
        match file.read_to_string(&mut cfg_contents) {
            Ok(s) => {
                log::info!("Configuration file read_to_string: {}", s);
                s
            }
            Err(e) => panic!("Failed to read configuration file, error message:{}", e),
        };
        match serde_yaml::from_str(&cfg_contents) {
            Ok(c) => {
                log::info!("Configuration file from_str: {:?}", c);
                c
            }
            Err(e) => panic!("Failed to parse configuration file, error message:{}", e),
        }
    }
    /// get the config
    pub fn init_by_config_and_environment() -> Self {
        dotenv().ok();
        // 创建一个配置对象
        let mut settings = config::Config::default();
        // 从 YAML 文件加载配置
        settings.merge(config::File::with_name(CONFIG_FILE)).expect("load yaml configuration");
        // Load configuration from environment variables
        settings.merge(Environment::new()).expect("load configuration from environment");
        // Deserialize the merged configuration into your Configs struct
        let mut configs: Configs = settings.try_into().expect("load configuration for Configs");
        // 打印配置
        log::info!("------{:?}", configs);
        configs
    }
}

pub static CERT_KEY: Lazy<CertKey> = Lazy::new(get_cert_key);

pub struct CertKey {
    pub cert: Vec<u8>,
    pub key: Vec<u8>,
}

impl CertKey {
    pub fn new(cert: Vec<u8>, key: Vec<u8>) -> Self {
        Self { cert, key }
    }
}

fn get_cert_key() -> CertKey {
    let cert = get_string(&CFG.cert.cert);
    let key = get_string(&CFG.cert.key);
    CertKey::new(cert, key)
}

fn get_string<P: AsRef<Path>>(path: P) -> Vec<u8> {
    std::fs::read(path).expect("failed to read file")
}
