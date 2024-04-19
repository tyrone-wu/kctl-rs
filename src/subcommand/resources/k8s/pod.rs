use bpaf::Bpaf;

#[derive(Bpaf, Debug)]
#[allow(dead_code)]
pub struct Pod {
    #[bpaf(short, long)]
    /// TODO: pod name
    name: String,
    // other pod things
}
