use actix_web::HttpResponse;

use super::content_loader::add_component;

use super::content_loader::read_file;

pub async fn items() -> HttpResponse {
    // HTML file not being compiled in Rust app, so we can update html instantly and refresh browser. it's apply new changes
    let mut html_data = read_file("./templates/main.html");
    // put js in separate file, load HTML strin that has a {{JS}} tag in the script section
    let javascript_data= read_file("./js/main.js");
    let css_data= read_file("./css/main.css");
    let css_base_data= read_file("./css/base.css");

    html_data = html_data.replace("{{JS}}", &javascript_data);
    html_data = html_data.replace("{{CSS}}", &css_data);
    html_data = html_data.replace("{{BASE_CSS}}", &css_base_data);

    html_data = add_component(String::from("header"), html_data);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
