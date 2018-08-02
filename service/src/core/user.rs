use chrono::Duration;
use chrono::Utc;
use failure::{Fail, ResultExt};
use fina_model::core::NewOnetimeToken;
use fina_model::core::{
    NewUser, NewUserPassword, OnetimeToken, TokenType, User, UserPassword, UserSignUp,
};
use fina_model::error::DataError;
use fina_util::db::tx;
use fina_util::{argon2_hash, argon2_verify, error::Error, new_uuid, sha512, Context};
use fina_util::{error_from_unhandled};
use log::{debug, info, log};
use serde_derive::Serialize;
use fina_codegen::CustomError;


pub(crate) const SECRET_KEY: &str = "71ade6e0-51b1-4aa3-aa70-682ea7566d3f";
pub(crate) const PASSWORD_EXPIRY_DAYS: i64 = 365 * 25;
pub(crate) const USER_ACTIVATION_TOKEN_EXPIRY: i64 = 24;

pub fn create_user(context: &Context, new_user: &NewUser<'_>) -> Result<User, DataError> {
    let conn = context.db();
    tx(conn, |conn| User::insert(conn, new_user))
}

pub fn sign_up(context: &Context, user_ac: &UserSignUp) -> Result<User, SignUpError> {
    let conn = context.db();

    debug!("User to register {:?}", user_ac);

    let new_user = NewUser {
        first_name: user_ac.first_name.as_ref(),
        last_name: user_ac.last_name.as_ref(),
        username: user_ac.email.as_ref(),
        email: user_ac.email.as_ref(),
    };

    if User::exists_by_username(conn, user_ac.email.as_ref())? {
        Err(SignUpErrorKind::UserEmailAlreadyExists)?;
    }

    let user = User::insert(conn, &new_user)?;

    let password_sha512 = sha512(user_ac.password.clone().into_bytes().as_ref());
    let password_hash = argon2_hash(&password_sha512, SECRET_KEY.as_ref())?;
    let user_password = NewUserPassword {
        user_id: user.id,
        hash: password_hash,
        expiry_date: Utc::now() + Duration::days(PASSWORD_EXPIRY_DAYS),
    };

    let _ = UserPassword::insert(conn, &user_password)?;
    let uuid = new_uuid();
    let uuid_hash = argon2_hash(&uuid.as_ref(), SECRET_KEY.as_ref())?;
    let activation_token = NewOnetimeToken {
        user_id: Some(user.id),
        token_type: TokenType::UserActivation,
        token: uuid_hash,
        expiry_date: Utc::now() + Duration::hours(USER_ACTIVATION_TOKEN_EXPIRY),
    };
    let _ = OnetimeToken::insert(conn, &activation_token)?;
    send_activation_email(&user, &uuid);
    Ok(user)
}

pub fn activate(context: &Context, token: &str) -> Result<(), ActivationError> {
    let conn = context.db();
    let (ott, mut user) = OnetimeToken::find_user_and_token(conn, token)?
        .ok_or_else(|| ActivationErrorKind::InvalidToken)?; //User does not exists

    user.activated = true;
    User::activate(conn, &user)?;
    OnetimeToken::delete(conn, ott.id).map_err(ActivationError::to_internal_err)
}

fn send_activation_email(user: &User, token: &str) {



    info!("User:{:?}, activation code: {:?}", user, token);
}

pub fn login(context: &Context, username: &str, password: &str) -> Result<User, self::LoginError> {
    let conn = context.db();

    let mut user = User::find_by_username(conn, username)?
        .ok_or_else(|| LoginErrorKind::InvalidUsernameOrPassword)?;
    let up = UserPassword::find(conn, user.id)
        .map_err(LoginError::to_internal_err)?
        .ok_or_else(|| LoginErrorKind::InvalidUsernameOrPassword)?;

    let password_sha512 = sha512(password.as_ref());
    let valid = argon2_verify(&password_sha512, SECRET_KEY.as_ref(), &up.hash)
        .context(LoginErrorKind::Internal)?;

    if !valid {
        user.failed_logins += 1;
        let _ = User::update(conn, &user)?;
        Err(LoginErrorKind::InvalidUsernameOrPassword)?;
    } else if !user.activated {
        Err(LoginErrorKind::AccountNotYetActivated)?;
    } else if user.locked {
        Err(LoginErrorKind::AccountLocked)?;
    }

    Ok(user)
}

pub fn find_by_id(context: &Context, id: i64) -> Result<User, DataError> {
    let conn = context.db();
    User::find(conn, id)
}


pub fn map_to<T, E>(error_kind: E) -> impl Fn(T) -> ::failure::Context<E>
where
    T: Into<::failure::Error>,
    E: ::failure::Fail + Copy,
{
    move |err| err.into().context(error_kind)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize, CustomError)]
pub enum SignUpErrorKind {
    #[fail(display = "User already exists with same email")]
    UserEmailAlreadyExists,

    #[fail(display = "Internal error")]
    #[custom_error(map_from(DataError,Error))]
    Internal,
}
//error_from_unhandled!(SignUpError,SignUpErrorKind,DataError);
//error_from_unhandled!(SignUpError,SignUpErrorKind,failure::Error);


#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize, CustomError)]
pub enum ActivationErrorKind {
    #[fail(display = "Invalid activation token")]
    InvalidToken,

    #[fail(display = "Account is currently locked")]
    AccountLocked,

    #[fail(display = "Internal error")]
    #[custom_error(map_from(DataError))]
    Internal,
}

//error_from_unhandled!(ActivationError,ActivationErrorKind,DataError);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize,CustomError)]
pub enum LoginErrorKind {
    #[fail(display = "Invalid activation token")]
    InvalidUsernameOrPassword,

    #[fail(display = "Account is currently locked")]
    AccountLocked,

    #[fail(display = "Account is not activated")]
    AccountNotYetActivated,

    #[fail(display = "Internal error")]
    #[custom_error(map_from(DataError))]
    Internal,
}
