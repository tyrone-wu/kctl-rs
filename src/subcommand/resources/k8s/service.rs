use bpaf::Bpaf;

#[derive(Bpaf, Debug)]
#[allow(dead_code)]
pub struct Service {
    #[bpaf(short, long)]
    /// TODO: service name
    name: String,
    // other service things
}
