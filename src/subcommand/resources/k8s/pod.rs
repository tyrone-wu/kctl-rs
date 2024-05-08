use bpaf::Bpaf;

use super::resource_definition::{ResourceDefinition, Scope};

#[derive(Bpaf, Debug)]
pub struct Pod {
    #[bpaf(short, long)]
    /// TODO: pod name
    name: Option<String>,

    #[bpaf(long, long("ns"), fallback("default".to_string()), display_fallback)]
    /// TODO: namespace
    namespace: String,
}

impl Pod {
    pub fn get_path(&self) -> String {
        let resource = ResourceDefinition::new(Scope::Namespace(self.namespace.clone()));
        let mut path: String = resource.get_path();
        path.push_str("/pods/");
        if let Some(name) = &self.name {
            path.push_str(name);
        }
        println!("{path}");
        path
    }
}
