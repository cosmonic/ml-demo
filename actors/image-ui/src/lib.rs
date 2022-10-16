use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{
    HttpRequest, HttpResponse, HttpServer, HttpServerReceiver, HttpServerSender,
};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct ImageUiActor {}

const INDEX_HTML: &str = include_str!("../index.html");

#[async_trait]
impl HttpServer for ImageUiActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let path = req.path.trim().trim_end_matches('/');
        if req.method == "GET" && path.is_empty() {
            let mut resp = HttpResponse {
                body: INDEX_HTML.as_bytes().to_vec(),
                ..Default::default()
            };
            resp.header
                .insert("content-type".into(), vec!["text/html".into()]);
            Ok(resp)
        } else {
            // forward to Inference API
            HttpServerSender::to_actor("inferenceapi")
                .handle_request(ctx, req)
                .await
        }
    }
}
