use super::jwt;
use actix_web::dev::ServiceRequest;

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

#[cfg(test)]
mod check_credentials_tests {
    use super::super::jwt::JwtToken;
    use super::check_password;
    use super::extract_header_token;
    use actix_web;
    #[test]
    fn correct_check_password() {}
    #[test]
    fn incorrect_check_password() {}
    #[test]
    fn no_token_in_extract_header_token() {}
    #[test]
    fn correct_token_in_extract_header_token() {}
}
