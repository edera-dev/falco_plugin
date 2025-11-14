use super::dial::ControlDialAddress;
#[cfg(unix)]
use super::unix::HyperUnixConnector;
use crate::proto::generated::protect::control::v1::control_service_client::ControlServiceClient;
#[cfg(not(unix))]
use anyhow::anyhow;
use anyhow::{Result, bail};
use tonic::transport::{Channel, Endpoint};

pub struct ControlClientProvider {}

impl ControlClientProvider {
    pub async fn dial(addr: ControlDialAddress) -> Result<ControlServiceClient<Channel>> {
        let channel = match addr {
            ControlDialAddress::UnixSocket { path } => {
                #[cfg(not(unix))]
                bail!(
                    "unix sockets are not supported on this platform (path {})",
                    path
                );
                #[cfg(unix)]
                ControlClientProvider::dial_unix_socket(path).await?
            }
            _ => {
                bail!("only unix socket connections are supported with this client");
            }
        };

        Ok(ControlServiceClient::new(channel))
    }

    #[cfg(unix)]
    async fn dial_unix_socket(path: String) -> Result<Channel> {
        // This URL is not used but is required to be specified.
        Ok(Endpoint::try_from(format!("unix://localhost/{path}"))?
            .connect_with_connector(HyperUnixConnector::new(path))
            .await?)
    }
}
