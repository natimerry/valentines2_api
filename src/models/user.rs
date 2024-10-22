use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};



#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct AuthUserSignupRequest {
    #[validate(
        length(min = 3, message = "Username required to be more than 3 characters"),
        custom(function = "validate_username")
    )]
    pub username: String,
    #[validate(length(min = 8, message = "Password required to be more than 8 characters"))]
    pub password: String,
    // pub token: String
    #[validate(email)]
    pub email: String,
}

pub struct User{
    
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.contains(":")
        || username.contains("-")
        || username.contains("@")
        || username.contains("-")
        || username.contains(">")
        || username.contains(' ')
    {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("SpecialChar"));
    }

    Ok(())
}