// use bpaf::{construct, positional, short, Parser};

// use super::resource_type::ResourceType;

// #[derive(Debug)]
// #[allow(dead_code)]
// pub struct ResourceDefinition {
//     name: String,
// }

// pub fn parse_resource() -> impl Parser<ResourceDefinition> {
//     let name = short('n')
//         .long("name")
//         .argument::<String>("NAME")
//         .help("TODO: resource name");

//     construct!(ResourceDefinition { name })
// }
