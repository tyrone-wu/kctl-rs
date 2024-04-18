use bpaf::Bpaf;

use super::methods::{
    delete::{delete_request, DeleteRequest},
    get::{get_request, GetRequest},
    patch::{patch_request, PatchRequest},
    post::{post_request, PostRequest},
    put::{put_request, PutRequest},
};

#[derive(Bpaf, Debug)]
#[bpaf(group_help("Available commands:"))]
pub enum Request {
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
