#[macro_use]
extern crate actix_web;

use std::{
    cell::Cell,
    sync::{Arc, Mutex, atomic::AtomicUsize},
};

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Result,
    error::{Error, InternalError, JsonPayloadError},
    middleware, web,
};

// use `Serde` to turn JSON data with that format into instances of our struct
// Serialize: convert Rust type to JSON data
// Deserialize: convert JSON data to Rust type
use serde::{Deserialize, Serialize};

pub struct MessageApp {
    port: u16,
}

const LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let messages = Arc::new(Mutex::new(vec![]));
        println!("Starting http server: 127.0.0.1:{}", self.port);

        HttpServer::new(move || {
            App::new()
                .data(AppState {
                    server_id: SERVER_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                    request_count: Cell::new(0),
                    messages: messages.clone(),
                })
                .wrap(middleware::Logger::new(LOG_FORMAT))
                .service(index)
                .service(
                    web::resource("/send")
                        .data(
                            web::JsonConfig::default()
                                .limit(4096)
                                .error_handler(post_error),
                        )
                        .route(web::post().to(post)),
                )
                .service(clear)
        })
        .bind(("127.0.0.1", self.port))? // ? operator return Result<Ok, Err> - if return Result<Err> return early with that value
        .workers(8)
        .run()
    }
}

#[derive(Serialize)] // we call it attribute, derive(suy ra) thuo.c ti'nh Serialize of serde
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    message: Vec<String>,
}

#[get("/")]
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        message: ms.clone(),
    }))
}

static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);
struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

#[derive(Deserialize)]
struct PostInput {
    message: String,
}

#[derive(Serialize)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}

fn post(msg: web::Json<PostInput>, state: web::Data<AppState>) -> Result<web::Json<PostResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut ms = state.messages.lock().unwrap();
    ms.push(msg.message.clone());

    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count,
        message: msg.message.clone(),
    }))
}

#[post("/clear")]
fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut ms = state.messages.lock().unwrap();
    ms.clear();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        message: vec![],
    }))
}

#[derive(Serialize)]
struct PostError {
    server_id: usize,
    request_count: usize,
    error: String,
}
fn post_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
    let state = req.app_data::<AppState>().unwrap(); // .unwrap(): OK or panic! macro
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let post_error = PostError {
        server_id: state.server_id,
        request_count,
        error: format!("{}", err),
    };
    InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into()
}
