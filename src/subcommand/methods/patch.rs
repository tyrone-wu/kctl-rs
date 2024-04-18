use bpaf::Bpaf;

use crate::subcommand::resources::resource_type::{resource_type, ResourceType};

#[derive(Bpaf, Debug)]
#[allow(dead_code)]
pub struct PatchRequest {
    #[bpaf(switch, long("TODO-patch"))]
    /// TODO: patch options description
    opts: bool,

    // more patch options
    #[bpaf(
        external(resource_type),
        custom_usage("RESOURCE"),
        group_help("Available resources:")
    )]
    resource_type: ResourceType,
}
