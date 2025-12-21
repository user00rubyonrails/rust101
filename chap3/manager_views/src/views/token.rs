use actix_web::dev::ServiceRequest;

// check password
fn check_password(password: String) -> Result<String, &'static str> {
    if password == "token" {
        return Ok(password)
    }

    return Err("token not authorised")
}

// Extract token from `header` func
fn extract_header_token(request: &ServiceRequest)  -> Result<String, &'static str>{
    match request.headers().get("user-token") {
        Some(token) => {
            match token.to_str() {
                Ok(proccessed_password) => Ok(String::from(proccessed_password)),
                Err(_proccessed_password) => Err("There was an error processing token"),
            }
        },
        None => Err("There is no token")
    }
}

pub fn process_token(request: &ServiceRequest) -> Result<String, &'static str> {
    match extract_header_token(request) {
        Ok(token) => check_password(token),
        Err(msg) => Err(msg)
    }
}
