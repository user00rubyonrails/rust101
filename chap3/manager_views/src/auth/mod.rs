use actix_web::dev::ServiceRequest;

pub mod jwt;
mod processes;

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    println!("❌----process_token-----------");
    match processes::extract_header_token(request) {
        Ok(token) => match processes::check_password(token){
            Ok(token) => Ok(token),
            Err(msg) => Err(msg),
        },
        Err(msg) => Err(msg),
    }
}
