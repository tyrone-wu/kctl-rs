use std::{env, fmt::Display, fs, str::FromStr};

use base64::{prelude::BASE64_STANDARD, Engine};
use kube::config::Kubeconfig;
use secrecy::ExposeSecret;

#[derive(Debug)]
pub struct Auth {
    path: String,
    api_server: String,
    cert: String,
    key: String,
}

impl Auth {
    fn new(path: &str) -> Self {
        let file = fs::read(path).expect("Unable to read kubeconfig file");
        let content = String::from_utf8_lossy(&file);
        let config: Kubeconfig =
            serde_yaml::from_str(&content).expect("Unable to parse config file into kubeconfig");

        // Get current context if exists, otherwise get first context in the list
        let context = config
            .current_context
            .map(|current_context| {
                let named_context = config.contexts.iter().find(|ctx| ctx.name == current_context);
                let context_opt = if named_context.is_some() {
                    named_context
                } else {
                    config.contexts.first()
                };

                context_opt
                    .expect("No context found")
                    .context
                    .as_ref()
                    .expect("No context found")
            })
            .unwrap();

        // Parse api server
        let cluster = config
            .clusters
            .iter()
            .find(|cluster| cluster.name == context.cluster)
            .expect("No cluster found")
            .cluster
            .as_ref()
            .expect("No cluster found");
        let api_server = cluster.server.to_owned().expect("Server not found");

        // Parse cert and key
        let user = config
            .auth_infos
            .iter()
            .find(|user| user.name == context.user)
            .expect("No user found")
            .auth_info
            .as_ref()
            .expect("No user found");

        let cert_secret = user
            .client_certificate_data
            .as_ref()
            .expect("No client cert data found");
        let cert = String::from_utf8(
            BASE64_STANDARD
                .decode(cert_secret)
                .expect("Unable to decode cert"),
        )
        .expect("Unable to decode cert");

        let key_secret = user
            .client_key_data
            .as_ref()
            .expect("No client key data found")
            .expose_secret();
        let key = String::from_utf8(BASE64_STANDARD.decode(key_secret).expect("Unable to decode key"))
            .expect("Unable to decode key");

        Self {
            path: path.to_owned(),
            api_server,
            cert,
            key,
        }
    }
}

impl Display for Auth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl FromStr for Auth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("No kube config file found.".to_string());
        }
        Ok(Auth::new(s))
    }
}

pub fn fallback_kubeconfig() -> Result<Auth, String> {
    let path = env::var("HOME").expect("HOME env does not exist");
    Auth::from_str(&format!("{path}/.kube/config"))
}
