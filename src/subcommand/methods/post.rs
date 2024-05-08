use bpaf::Bpaf;

use crate::subcommand::resources::resource_type::{resource_type, ResourceType};

#[derive(Bpaf, Debug)]
#[allow(dead_code)]
pub struct PostRequest {
    #[bpaf(switch, long("TODO-post"))]
    /// TODO: post options description
    opts: bool,

    // more post options?
    #[bpaf(
        external(resource_type),
        custom_usage("RESOURCE"),
        group_help("Available resources:")
    )]
    pub resource_type: ResourceType,
}
