use bpaf::Bpaf;
use reqwest::ClientBuilder;

use crate::subcommand::resources::resource_type::{resource_type, ResourceType};

#[derive(Bpaf, Debug)]
pub struct GetRequest {
    #[bpaf(switch, long("TODO-get"))]
    /// TODO: get options description
    opts: bool,

    #[bpaf(
        external(resource_type),
        custom_usage("RESOURCE"),
        group_help("Available resources:")
    )]
    pub resource_type: ResourceType,
}

impl GetRequest {
    pub async fn send(
        &self,
        api_server: &str,
        client: ClientBuilder,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let GetRequest {
            opts: _opts,
            resource_type,
        } = &self;
        let output = resource_type.get_request(api_server, client).await?;
        println!("{}", output);
        Ok(())
    }
}
