pub mod schema;
mod sites;

use std::collections::HashMap;

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};
use maud::html;
use reqwest::multipart::Form;

#[derive(Clone)]
struct App {
    runtime: orb_runtime::Runtime,
}

#[tokio::main]
async fn main() {
    let runtime = orb_runtime::Runtime::new();

    let app = App { runtime };

    let router = Router::new()
        .route("/", get(upload))
        .route("/", post(upload_wasm))
        .route("/test", get(tess))
        .with_state(app);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn tess(State(app): State<App>) -> Html<String> {
    Html(app.runtime.get_wasm_string("test".to_string()).unwrap())
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
            div class="container" {
                h4 { "WASM Test" }
                form method="post" enctype="multipart/form-data" hx-post="/" hx-target="#result" {
                    input type="text" name="name" placeholder="Name";
                    input type="file" name="file" multiple="multiple" _="on htmx:xhr:process(loaded, total) set #progress.value = loaded / total";
                    progress id="progress" value="0" max="100" {}
                    input type="submit" value="Upload";
                }
                button hx-get="/test" hx-target="#result" { "Test me" }
                p id="result" {}
            }
        }
    }).into_string())
}

async fn upload_to_ipfs(data: Vec<u8>) -> Result<(), StatusCode> {
    let client = reqwest::Client::new();

    let res = client
        .post("http://localhost:5001/api/v0/key/list")
        // .query(&[("arg", "/namewasm"), ("create", "true")])
        // .multipart(Form::new().part("file", reqwest::multipart::Part::bytes(data)))
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        return Err(res.status());
    }
    println!("{:?}", res.text().await.unwrap());

    Ok(())
}

async fn upload_wasm(
    State(mut app): State<App>,
    mut data: Multipart,
) -> Result<Html<String>, StatusCode> {

    struct Params {
        name: String,
        file: Vec<u8>,
    }

    let mut params = Params {
        name: String::new(),
        file: Vec::new(),
    };

    while let Some(field) = data.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "name" => params.name = field.text().await.unwrap(),
            "file" => params.file = field.bytes().await.unwrap().to_vec(),
            _ => (),
        }
    }

    if params.file.is_empty() || params.name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    app.runtime
        .add_module(&params.name, &params.file)
        .unwrap();

    let resp = upload_to_ipfs(params.file.to_vec()).await;

    if resp.is_err() {
        return Err(resp.unwrap_err());
    }

    let markup = html! {
        ins { "test" }
    };

    Ok(Html(markup.into_string()))
}
