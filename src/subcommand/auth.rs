use std::{env, fmt::Display, fs, str::FromStr};

use base64::{prelude::BASE64_STANDARD, Engine};
use kube::config::Kubeconfig;
use reqwest::{Certificate, Identity};
use secrecy::ExposeSecret;

#[derive(Debug)]
pub struct Auth {
    path: String,
    pub api_server: String,
    ca_cert: Vec<u8>,
    client_cert: Vec<u8>,
    client_key: Vec<u8>,
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
            .expect("No context found");

        // Parse cluster ca and api server
        let cluster = config
            .clusters
            .iter()
            .find(|cluster| cluster.name == context.cluster)
            .expect("No cluster found")
            .cluster
            .as_ref()
            .expect("No cluster found");
        let api_server = cluster.server.to_owned().expect("Server not found");

        let ca_cert_secret = cluster
            .certificate_authority_data
            .as_ref()
            .expect("No ca cert data found");
        let ca_cert = BASE64_STANDARD
            .decode(ca_cert_secret)
            .expect("Unable to decode ca cert");

        // Parse client cert and key
        let user = config
            .auth_infos
            .iter()
            .find(|user| user.name == context.user)
            .expect("No user found")
            .auth_info
            .as_ref()
            .expect("No user found");

        let client_cert_secret = user
            .client_certificate_data
            .as_ref()
            .expect("No client client cert data found");
        let client_cert = BASE64_STANDARD
            .decode(client_cert_secret)
            .expect("Unable to decode client cert");

        let client_key_secret = user
            .client_key_data
            .as_ref()
            .expect("No client key data found")
            .expose_secret();
        let client_key = BASE64_STANDARD
            .decode(client_key_secret)
            .expect("Unable to decode client key");

        Self {
            path: path.to_string(),
            api_server,
            ca_cert,
            client_cert,
            client_key,
        }
    }

    pub fn get_mtls(&self) -> Result<(Certificate, Identity), Box<dyn std::error::Error>> {
        let ca_cert = Certificate::from_pem(&self.ca_cert).expect("Unable to parse ca cert");

        let mut client = self.client_cert.clone();
        client.extend_from_slice(&self.client_key);
        let client_ident = Identity::from_pem(&client).expect("Unable to parse client cert and key");

        Ok((ca_cert, client_ident))
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
