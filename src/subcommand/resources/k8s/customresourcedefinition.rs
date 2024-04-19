use bpaf::Bpaf;

#[derive(Bpaf, Debug)]
#[allow(dead_code)]
pub struct CustomResourceDefinition {
    #[bpaf(short, long)]
    /// TODO: crd name
    name: String,
    // other crd things
}
