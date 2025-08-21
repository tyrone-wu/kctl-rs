use std::time::Duration;

use bpaf::Bpaf;
use reqwest::Client;
use tracing::info;

use super::{
    auth::Auth,
    resource_cmd::{resource_cmd, ResourceCmd},
};

#[derive(Bpaf, Debug)]
pub enum Method {
    #[bpaf(command("get"))]
    /// TODO: get request description
    Get {
        #[bpaf(switch, long("placeholder"))]
        /// TODO: get options description
        placeholder: bool,

        #[bpaf(
            external(resource_cmd),
            custom_usage("RESOURCE"),
            group_help("Available resources:")
        )]
        resource_type: ResourceCmd,
    },

    #[bpaf(command("post"))]
    /// TODO: post request description
    Post {
        #[bpaf(switch, long("placeholder"))]
        /// TODO: post options description
        placeholder: bool,

        #[bpaf(
            external(resource_cmd),
            custom_usage("RESOURCE"),
            group_help("Available resources:")
        )]
        resource_type: ResourceCmd,
    },

    #[bpaf(command("put"))]
    /// TODO: put request description
    Put {
        #[bpaf(switch, long("placeholder"))]
        /// TODO: put options description
        placeholder: bool,

        #[bpaf(
            external(resource_cmd),
            custom_usage("RESOURCE"),
            group_help("Available resources:")
        )]
        resource_type: ResourceCmd,
    },

    #[bpaf(command("delete"))]
    /// TODO: delete request description
    Delete {
        #[bpaf(switch, long("placeholder"))]
        /// TODO: delete options description
        placeholder: bool,

        // more del options
        #[bpaf(
            external(resource_cmd),
            custom_usage("RESOURCE"),
            group_help("Available resources:")
        )]
        resource_type: ResourceCmd,
    },

    #[bpaf(command("patch"))]
    /// TODO: patch request description
    Patch {
        #[bpaf(switch, long("placeholder"))]
        /// TODO: patch options description
        placeholder: bool,

        // more patch options
        #[bpaf(
            external(resource_cmd),
            custom_usage("RESOURCE"),
            group_help("Available resources:")
        )]
        resource_type: ResourceCmd,
    },
}

impl Method {
    pub async fn send(&self, verbose: bool, auth: &Auth) -> Result<(), Box<dyn std::error::Error>> {
        let timeout: Duration = Duration::from_secs(2);
        let (ca_cert, client_ident) = auth.get_mtls(verbose)?;

        let client = Client::builder()
            .timeout(timeout)
            .use_rustls_tls()
            .add_root_certificate(ca_cert)
            .identity(client_ident);

        match &self {
            Method::Get {
                placeholder: _placeholder,
                resource_type,
            } => {
                let output = resource_type.get_request(&auth.api_server, client).await?;
                if verbose {
                    info!("Request: GET {}", output);
                }
            }
            Method::Post { .. } => todo!(),
            Method::Put { .. } => todo!(),
            Method::Delete { .. } => todo!(),
            Method::Patch { .. } => todo!(),
        }

        Ok(())
    }
}
