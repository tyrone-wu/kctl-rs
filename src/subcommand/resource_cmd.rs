use bpaf::Bpaf;
use reqwest::ClientBuilder;

use super::api::{
    api_group::{api, Api},
    resources::{
        namespace::{namespace, Namespace},
        namespaced::{
            pod::{pod, Pod},
            service::{service, Service},
        },
    },
};

#[derive(Bpaf, Debug)]
pub enum ResourceCmd {
    #[bpaf(command("api"))]
    /// TODO: api resources description
    Api(#[bpaf(external(api))] Api),

    #[bpaf(command("namespace"), long("ns"))]
    /// TODO: namespace type description
    Namespace(#[bpaf(external(namespace))] Namespace),

    #[bpaf(command("pod"), long("po"))]
    /// TODO: pod type description
    Pod(#[bpaf(external(pod))] Pod),

    #[bpaf(command("service"), long("svc"))]
    /// TODO: service type description
    Service(#[bpaf(external(service))] Service),
}

impl ResourceCmd {
    pub async fn get_request(
        &self,
        api_server: &str,
        client: ClientBuilder,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = client.build()?;
        let url = match &self {
            ResourceCmd::Api(api) => api.get_request(api_server, client).await?,
            ResourceCmd::Namespace(namespace) => namespace.get_request(api_server, client).await?,
            ResourceCmd::Pod(pod) => pod.get_request(api_server, client).await?,
            ResourceCmd::Service(_) => todo!(),
        };
        Ok(url)
    }
}
