use bpaf::Bpaf;

use crate::subcommand::resources::resource_type::{resource_type, ResourceType};

#[derive(Bpaf, Debug)]
#[allow(dead_code)]
pub struct PutRequest {
    #[bpaf(switch, long("TODO-put"))]
    /// TODO: put options description
    opts: bool,

    // more put options?
    #[bpaf(
        external(resource_type),
        custom_usage("RESOURCE"),
        group_help("Available resources:")
    )]
    resource_type: ResourceType,
}
