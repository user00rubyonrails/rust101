use actix_web::HttpResponse;

use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    // HTML file not being compiled in Rust app, so we can update html instantly and refresh browser. it's apply new changes
    let html_data = read_file("./templates/main.html");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
