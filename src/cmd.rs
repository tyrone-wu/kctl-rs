use bpaf::Bpaf;

use crate::subcommand::{
    auth::{fallback_kubeconfig, Auth},
    request::{method, Method},
};

#[derive(Bpaf, Debug)]
#[bpaf(options)]
pub struct Options {
    #[bpaf(switch, short, long)]
    /// TODO: verbose description
    pub verbose: bool,

    #[bpaf(
        argument("KUBECONFIG"),
        short,
        long,
        fallback_with(|| fallback_kubeconfig()),
        display_fallback
    )]
    /// TODO: kubeconfig description
    pub config: Auth,

    #[bpaf(external(method), custom_usage("COMMANDS"), group_help("Available commands:"))]
    pub request: Method,
}
