use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use url::Url;

#[derive(Clone)]
pub enum ControlDialAddress {
    UnixSocket {
        path: String,
    },
    Tcp {
        host: String,
        port: u16,
    },
    Tls {
        host: String,
        port: u16,
        insecure: bool,
    },
}

impl FromStr for ControlDialAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url: Url = s.parse()?;

        match url.scheme() {
            "unix" => Ok(ControlDialAddress::UnixSocket {
                path: url.path().to_string(),
            }),

            _ => Err(anyhow!("unknown control address scheme: {}", url.scheme())),
        }
    }
}

impl From<ControlDialAddress> for Url {
    fn from(val: ControlDialAddress) -> Self {
        match val {
            ControlDialAddress::UnixSocket { path } => {
                let mut url = Url::parse("unix:///").unwrap();
                url.set_path(&path);
                url
            }

            _ => {
                panic!("unsupported URL scheme");
            }
        }
    }
}

impl Display for ControlDialAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url: Url = self.clone().into();
        write!(f, "{url}")
    }
}
