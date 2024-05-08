use bpaf::Bpaf;

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
    pub fn send(&self) -> (&str, &str, &str) {
        let GetRequest {
            opts: _opts,
            resource_type,
        } = &self;

        resource_type.process()
    }
}
