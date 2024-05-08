use bpaf::Bpaf;

use super::k8s::{
    customresourcedefinition::{custom_resource_definition, CustomResourceDefinition},
    pod::{pod, Pod},
    service::{service, Service},
};

#[derive(Bpaf, Debug)]
pub enum ResourceType {
    #[bpaf(command("pod"), long("po"))]
    /// TODO: pod type description
    Pod(#[bpaf(external(pod))] Pod),

    #[bpaf(command("service"), long("svc"))]
    /// TODO: service type description
    Service(#[bpaf(external(service))] Service),

    #[bpaf(command("customresourcedefinition"), long("crd"))]
    /// TODO: crd type description
    CustomResourceDefinition(#[bpaf(external(custom_resource_definition))] CustomResourceDefinition),
}

impl ResourceType {
    pub fn get_path(&self) -> String {
        match &self {
            ResourceType::Pod(pod) => pod.get_path(),
            ResourceType::Service(_) => todo!(),
            ResourceType::CustomResourceDefinition(_) => todo!(),
        }
    }
}
