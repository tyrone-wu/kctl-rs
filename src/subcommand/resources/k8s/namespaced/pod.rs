use bpaf::Bpaf;
use k8s_openapi::{api::core::v1::Pod as K8sPod, List};
use reqwest::Client;

use crate::subcommand::resources::k8s::namespace::Namespace;

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
    fn get_path(&self) -> String {
        let namespace = Namespace::new(self.namespace.clone());
        let mut path = namespace.get_path();
        if let Some(name) = &self.name {
            path.push_str(&format!("/{name}"));
        }
        path
    }

    pub async fn get_request(
        &self,
        api_server: &str,
        client: Client,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let path = &self.get_path();
        let url = format!("{api_server}/{path}");
        let response = client.get(url).send().await?;
        if self.name.is_some() {
            let body = response.json::<List<K8sPod>>().await?;
            println!("{:#?}", body);
        } else {
            let body = response.json::<K8sPod>().await?;
            println!("{:#?}", body);
        };

        Ok("asdf".to_string())
    }
}
