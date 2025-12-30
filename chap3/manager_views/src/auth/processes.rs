use actix_web::dev::ServiceRequest;
use super::jwt;

type ResultType = Result<String, &'static str>;
// check password
pub fn check_password(password: String) -> ResultType {
    match jwt::JwtToken::decode(password) {
        Ok(_token) => Ok(String::from("passed")),
        Err(msg) => Err(msg),
    }
    // if password == "token" {
    //     return Ok(password);
    // }

    // return Err("token not authorised");
}

// Extract token from `header` func
pub fn extract_header_token(request: &ServiceRequest) -> ResultType {
    match request.headers().get("user-token") {
        Some(token) => match token.to_str() {
            Ok(proccessed_password) => Ok(String::from(proccessed_password)),
            Err(_proccessed_password) => Err("There was an error processing token"),
        },
        None => Err("There is no token"),
    }
}
