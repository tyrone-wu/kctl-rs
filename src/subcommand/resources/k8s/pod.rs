use bpaf::Bpaf;

// use crate::subcommand::request::Request;

#[derive(Bpaf, Debug)]
pub struct Pod {
    #[bpaf(short, long)]
    /// TODO: pod name
    name: Option<String>,
    // other pod things
}

impl Pod {
    pub fn process(&self) -> (&str, &str, &str) {
        ("v1", "default", "pod")
    }
}
