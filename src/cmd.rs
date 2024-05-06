use bpaf::Bpaf;

use crate::subcommand::{
    auth::{fallback_kubeconfig, Auth},
    request::{request, Request},
};

#[derive(Bpaf, Debug)]
#[bpaf(options)]
#[allow(dead_code)]
pub struct Options {
    #[bpaf(switch, short, long)]
    /// TODO: verbose description
    verbose: bool,

    #[bpaf(
        argument("KUBECONFIG"),
        short,
        long,
        fallback_with(|| fallback_kubeconfig()),
        display_fallback
    )]
    /// TODO: kubeconfig description
    config: Auth,

    #[bpaf(external(request))]
    request: Request,
}
