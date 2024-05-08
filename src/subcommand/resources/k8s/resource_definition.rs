pub enum Scope {
    Namespace(String),
    Cluster,
}

impl ToString for Scope {
    fn to_string(&self) -> String {
        match &self {
            Scope::Namespace(namespace) => format!("namespaces/{}", namespace),
            Scope::Cluster => todo!(),
        }
    }
}

pub struct ResourceDefinition {
    version: String,
    scope: Scope,
}

impl<'a> ResourceDefinition {
    pub fn new(scope: Scope) -> Self {
        Self {
            version: "v1".to_string(),
            scope,
        }
    }

    pub fn get_path(&self) -> String {
        format!("/api/{}/{}", self.version, self.scope.to_string())
    }
}
