use bpaf::Bpaf;

use crate::subcommand::request::{request, Request};

#[derive(Bpaf, Debug)]
#[bpaf(options)]
#[allow(dead_code)]
pub struct Options {
    #[bpaf(switch, short, long)]
    /// TODO: verbose description
    verbose: bool,

    #[bpaf(switch, short, long)]
    /// TODO: kubeconfig description
    /// placeholder for now
    config: bool,

    #[bpaf(external(request))]
    request: Request,
}
