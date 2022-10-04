use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::{HttpClient, HttpClientSender};
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_numbergen::random_in_range;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct XkcdgeneratorActor {}

#[derive(serde::Deserialize)]
struct XkcdComic {
    title: String,
    img: String,
}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for XkcdgeneratorActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        _req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        let random_num = random_in_range(1, 2680).await?;
        let xkcd_url = format!("https://xkcd.com/{}/info.0.json", random_num);

        let response = HttpClientSender::new()
            .request(
                ctx,
                &wasmcloud_interface_httpclient::HttpRequest::get(&xkcd_url),
            )
            .await?;

        let comic: XkcdComic = serde_json::from_slice(&response.body).map_err(|e| {
            RpcError::ActorHandler(format!("Failed to deserialize comic request: {}", e))
        })?;

        let body = format!(
            r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Your XKCD random comic</title>
        </head>
        <body>
            <h1>{}</h1>
            <img src="{}"/>
        </body>
        </html>
            "#,
            comic.title, comic.img
        );

        Ok(HttpResponse {
            body: body.as_bytes().to_vec(),
            ..Default::default()
        })
    }
}
