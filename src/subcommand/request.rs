use std::time::Duration;

use bpaf::Bpaf;
use reqwest::Client;

use super::{
    auth::Auth,
    methods::{
        delete::{delete_request, DeleteRequest},
        get::{get_request, GetRequest},
        patch::{patch_request, PatchRequest},
        post::{post_request, PostRequest},
        put::{put_request, PutRequest},
    },
};

#[derive(Bpaf, Debug)]
pub enum Method {
    #[bpaf(command("get"))]
    /// TODO: get request description
    Get(#[bpaf(external(get_request))] GetRequest),

    #[bpaf(command("post"))]
    /// TODO: post request description
    Post(#[bpaf(external(post_request))] PostRequest),

    #[bpaf(command("put"))]
    /// TODO: put request description
    Put(#[bpaf(external(put_request))] PutRequest),

    #[bpaf(command("delete"))]
    /// TODO: delete request description
    Delete(#[bpaf(external(delete_request))] DeleteRequest),

    #[bpaf(command("patch"))]
    /// TODO: patch request description
    Patch(#[bpaf(external(patch_request))] PatchRequest),
}

impl Method {
    pub async fn send(&self, auth: &Auth) -> Result<(), Box<dyn std::error::Error>> {
        let timeout: Duration = Duration::from_secs(2);
        let (ca_cert, client_ident) = auth.get_mtls()?;
        let verbose = false;

        let client = Client::builder()
            .timeout(timeout)
            .use_rustls_tls()
            .add_root_certificate(ca_cert)
            .identity(client_ident)
            .connection_verbose(verbose)
            .build()
            .expect("Unable to build client");

        let mut url = auth.api_server.to_string();
        let request = match &self {
            Method::Get(req) => {
                url.push_str(&req.send());
                client.get(url)
            }
            Method::Post(_req) => todo!(),
            Method::Put(_req) => todo!(),
            Method::Delete(_req) => todo!(),
            Method::Patch(_req) => todo!(),
        };

        let response = request.send().await.expect("Error sending request");
        let message = response.text().await.expect("Unable to parse message");
        println!("{:?}", message);

        Ok(())
    }
}
