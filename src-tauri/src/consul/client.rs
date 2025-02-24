use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

#[derive(Serialize, Deserialize)]
pub struct Host(String);
impl Host {
    pub fn new(host: String) -> Host {
        Host(host.to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Port(u16);
impl Port {
    pub fn new(port: u16) -> Result<Port, Box<dyn std::error::Error>> {
        if port < 1 {
            return Err("Port must be between 1 and 65535".into());
        }

        Ok(Port(port))
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConsulClient {
    pub name: String,
    pub host: Host,
    pub port: Port,
    pub scheme: Scheme,
}

impl ConsulClient {
    pub fn address(&self) -> String {
        format!("{}://{}:{}", match self.scheme {
            Scheme::HTTP => "http",
            Scheme::HTTPS => "https",
        }, self.host.0, self.port.0)
    }
}