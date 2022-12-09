# conf_from_env

This library automatically reads configurations from environment variables. 
The configuration is defined in one struct for configuration values and in 
one struct for secrets. The environment variables are expected to have the 
same name as the struct attributes in uppercase.

# Example
```` bash
LOG_LEVEL=info

REDIS_HOST=127.0.0.1
REDIS_USER=lala
REDIS_PASSWORD=lili
````

```` rust
use conf_from_env::{ConfigAndSecrets, SetStructFromEnv};

#[derive(SetStructFromEnv, Clone)]
pub struct Config {
    pub log_level: String,

    pub redis_host: String,
    pub redis_user: String,
    
    timeout: u32
}

#[derive(SetStructFromEnv, Clone)]
pub struct Secrets {
    pub redis_password: String,
}

pub type CoSe = ConfigAndSecrets<Config, Secrets>;

fn main() {
    let cose = CoSe::from_env();
    println!("config and secrets: {:?}", cose);
}
````