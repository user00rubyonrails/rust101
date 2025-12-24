use actix_web::HttpResponse;

use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    // HTML file not being compiled in Rust app, so we can update html instantly and refresh browser. it's apply new changes
    let html_data = read_file("./templates/main.html");
    // put js in separate file, load HTML strin that has a {{JAVASCRIPT}} tag in the script section
    let javascript_data= read_file("./js/main.js");

    let html_data = html_data.replace("{{JS}}", &javascript_data);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
