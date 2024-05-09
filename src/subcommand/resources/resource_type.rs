use bpaf::Bpaf;
use reqwest::ClientBuilder;

use super::k8s::{
    namespace::{namespace, Namespace},
    namespaced::{
        pod::{pod, Pod},
        service::{service, Service},
    },
};

#[derive(Bpaf, Debug)]
pub enum ResourceType {
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

impl ResourceType {
    pub async fn get_request(
        &self,
        api_server: &str,
        client: ClientBuilder,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let client = client.build()?;
        let output = match &self {
            ResourceType::Namespace(namespace) => namespace.get_request(api_server, client).await?,
            ResourceType::Pod(pod) => pod.get_request(api_server, client).await?,
            ResourceType::Service(_) => todo!(),
        };
        Ok(output)
    }
}
