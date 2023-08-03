mod ipfs_rfc_client;
mod app_subrouter;
mod config;

use std::{sync::Arc, collections::HashMap};

use tower::Layer;

use axum::{
    extract::{Multipart, State, Host},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router, body::Body,
};
use maud::html;
use orb_runtime::Runtime;
use tokio::sync::Mutex;

pub struct Orb {
    runtime: Runtime,
    router: Router
}

pub type RuntimeDb = Arc<Mutex<HashMap<String, Orb>>>;

#[tokio::main]
async fn main() {
    let runtime_db = Arc::new(Mutex::new(HashMap::new()));

    let sub_router = app_subrouter::SubrouterLayer::new(runtime_db.clone());

    let router = Router::new()
        .route("/", get(upload))
        .route("/", post(upload_wasm))
        .route("/wasm_test", get(wasm_test))
        .route("/ipfs_test", get(ipfs_test))
        .route("/host_test", get(host_test))
        .with_state(runtime_db);

    let app = axum::middleware::from_fn(sub_router).layer(router);


    println!("Listening on: 3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn host_test(Host(hostname): Host, req: axum::http::Request<Body>) -> Html<String> {
    Html(hostname)
}

async fn wasm_test(State(runtime): State<RuntimeDb>) -> Html<String> {
    Html(
        (*runtime)
            .lock()
            .await
            .get("test")
            .unwrap()
            .runtime
            .get_wasm_string("test".to_string())
            .unwrap(),
    )
}

async fn ipfs_test() -> Html<String> {
    Html("test".to_string())
}

fn html_base() -> maud::Markup {
    html! {
        html {
            head {
                title { "WASM Test" }
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css";
                script src="https://unpkg.com/htmx.org@1.9.4" {}
                script src="https://unpkg.com/hyperscript.org@0.9.9" {}
            }
        }
    }
}

async fn upload() -> Html<String> {
    Html((html! {
        (html_base())
        body {
            main class="container" {
                h4 { "WASM Test" }
                form method="post" enctype="multipart/form-data" hx-post="/" hx-target="#result" {
                    input type="text" name="name" placeholder="Name";
                    input type="text" name="endpoint" placeholder="Name";
                    input type="file" name="file" multiple="multiple" _="on htmx:xhr:process(loaded, total) set #progress.value to loaded / total";
                    progress id="progress" value="0" max="100" {}
                    input type="submit" value="Upload";
                }
                button hx-get="/wasm_test" hx-target="next .result" { "Wasm Test" }
                span.result _="on htmx:afterSwap wait 2s then transition my opacity to 0% over 1s then set my innerHTML to ''" {}
                button hx-get="/ipfs_test" hx-target="next .result" { "IPFS Test Save" }
                span.result _="on htmx:afterSwap wait 2s then transition my opacity to 0% over 1s then set my innerHTML to ''" {}
                button hx-get="/ipfs_test" hx-target="next .result" { "IPFS Test Load" }
                span.result _="on htmx:afterSwap wait 2s then transition my opacity to 0% over 1s then set my innerHTML to ''" {}

                article {
                    h4 { "Rendered From Wasm" }
                    div id="result" {}
                }
            }
        }
    }).into_string())
}

async fn upload_wasm(
    State(runtime): State<RuntimeDb>,
    mut data: Multipart,
) -> Result<Html<String>, StatusCode> {
    struct Params {
        name: String,
        file: Vec<u8>,
        endpoint: String,
    }

    let mut params = Params {
        name: String::new(),
        file: Vec::new(),
        endpoint: String::new(),
    };

    while let Some(field) = data.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "name" => params.name = field.text().await.unwrap(),
            "endpoint" => params.endpoint = field.text().await.unwrap(),
            "file" => params.file = field.bytes().await.unwrap().to_vec(),
            _ => (),
        }
    }

    if params.file.is_empty() || params.name.is_empty() || params.endpoint.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    (*runtime)
        .lock()
        .await
        .get_mut("test")
        .unwrap()
        .runtime
        .add_module(&params.name, &params.file)
        .unwrap();

    let result = ipfs_rfc_client::save_wasm(&params.name, &params.endpoint, params.file).await;

    match result {
        Ok(_) => (),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let markup = html! {
        ins { (params.name) " for " (params.endpoint) " uploaded successfully!" }
    };

    Ok(Html(markup.into_string()))
}
