use bpaf::Bpaf;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{APIGroupList, APIResourceList, APIVersions};
use reqwest::Client;

#[derive(Bpaf, Clone, Debug)]
pub enum NamedGroup {
    #[bpaf(short, long)]
    /// TODO: apps named group description
    Apps,

    #[bpaf(short, long)]
    /// TODO: network named group description
    Network,

    #[bpaf(short, long)]
    /// TODO: storage named group description
    Storage,

    #[bpaf(short, long)]
    /// TODO: rbac named group description
    Rbac,

    #[bpaf(short, long)]
    /// TODO: all named group description
    All,
}

impl NamedGroup {
    fn get_path(&self) -> &str {
        match &self {
            NamedGroup::Apps => "apps",
            NamedGroup::Network => "networking.k8s.io",
            NamedGroup::Storage => "storage.k8s.io",
            NamedGroup::Rbac => "rbac.authorization.k8s.io",
            NamedGroup::All => "",
        }
    }
}

impl Default for NamedGroup {
    fn default() -> Self {
        NamedGroup::All
    }
}

#[derive(Bpaf, Clone, Debug)]
pub enum ApiGroup {
    #[bpaf(command("core"))]
    /// TODO: core api group description
    Core,

    #[bpaf(command("named"))]
    /// TODO: named api group description
    Named(
        #[bpaf(
            external(named_group),
            custom_usage("NAMED-GROUP"),
            group_help("Available named groups:"),
            fallback(NamedGroup::All)
        )]
        NamedGroup,
    ),
}

impl ApiGroup {
    fn get_path(&self) -> String {
        match &self {
            ApiGroup::Core => "api".to_string(),
            ApiGroup::Named(named) => format!("apis/{}", named.get_path()),
        }
    }
}

#[derive(Bpaf, Debug)]
pub struct Api {
    #[bpaf(external(api_group))]
    /// TODO: api group description
    api_group: ApiGroup,
}

impl Api {
    pub fn new(api_group: ApiGroup) -> Self {
        Self { api_group }
    }

    pub fn get_path(&self) -> String {
        self.api_group.get_path()
    }

    pub async fn get_request(
        &self,
        api_server: &str,
        client: Client,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let path = &self.get_path();
        let url = format!("{api_server}/{path}");

        let response = client.get(url.clone()).header("Accept", "application/json;v=v2;g=apidiscovery.k8s.io;as=APIGroupDiscoveryList").send().await?;
        match &self.api_group {
            ApiGroup::Core => {
                let body = response.json::<APIGroupList>().await?;
                println!("{:#?}", body);
            }
            ApiGroup::Named(_named) => {
                let body = response.json::<APIGroupList>().await?;
                println!("{:#?}", body);
            }
        }

        Ok(url)
    }
}
