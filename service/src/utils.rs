use crate::errors::ErrorResponse;

pub fn verify(hash: &str, password: &str) -> Result<bool, ErrorResponse> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), &[], &[]).map_err(|err| {
        dbg!(err);
        ErrorResponse::Unauthorized("Password failure".to_string())
    })
}
