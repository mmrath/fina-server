use chrono::Duration;
use chrono::Utc;
use diesel::Connection;
use failure::{Fail, ResultExt};
use model::core::NewOnetimeToken;
use model::core::{
    NewUser, NewUserPassword, OnetimeToken, TokenType, User, UserPassword, UserSignUp,
};
use model::error::{DataError, DataErrorKind};
use util::DbConnection;
use util::{argon2_hash, argon2_verify, error::Error, new_uuid, sha512, Context};

//use util::error::{Error, ErrorKind, ResultExt, UserError};

pub(crate) const SECRET_KEY: &str = "71ade6e0-51b1-4aa3-aa70-682ea7566d3f";
pub(crate) const PASSWORD_EXPIRY_DAYS: i64 = 365 * 25;
pub(crate) const USER_ACTIVATION_TOKEN_EXPIRY: i64 = 24;

pub fn create_user(context: &Context, new_user: &NewUser) -> Result<User, DataError> {
    let conn = context.db();
    tx(conn, |conn| User::insert(conn, new_user))
}

pub fn tx<T: Sized, E: Fail + Error, F: FnOnce(&DbConnection) -> Result<T, E>>(
    conn: &DbConnection,
    f: F,
) -> Result<T, E> {
    use diesel::connection::TransactionManager;
    use diesel::Connection;

    let tm = conn.transaction_manager();
    let _ = tm.begin_transaction(conn).map_err(E::to_internal_err)?;
    let res = f(conn);

    match res {
        Err(ref e) => if e.is_internal_err() {
            tm.rollback_transaction(conn).map_err(E::to_internal_err)?;
        } else {
            tm.commit_transaction(conn).map_err(E::to_internal_err)?;
        },
        Ok(_) => tm.commit_transaction(conn).map_err(E::to_internal_err)?,
    }

    return res;
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

pub fn login(context: &Context, username: &str, password: &str) -> Result<User, LoginError> {
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

error_kind!(
    SignUpError,
    SignUpErrorKind,
    ::model::error::DataError,
    ::failure::Error
);
error_kind!(
    ActivationError,
    ActivationErrorKind,
    ::model::error::DataError
);
error_kind!(LoginError, LoginErrorKind, ::model::error::DataError);

pub fn map_to<T, E>(error_kind: E) -> impl Fn(T) -> ::failure::Context<E>
where
    T: Into<::failure::Error>,
    E: ::failure::Fail + Copy,
{
    move |err| err.into().context(error_kind)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize)]
pub enum SignUpErrorKind {
    #[fail(display = "User already exists with same email")]
    UserEmailAlreadyExists,

    #[fail(display = "Internal error")]
    Internal,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize)]
pub enum ActivationErrorKind {
    #[fail(display = "Invalid activation token")]
    InvalidToken,

    #[fail(display = "Account is currently locked")]
    AccountLocked,

    #[fail(display = "Internal error")]
    Internal,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail, Serialize)]
pub enum LoginErrorKind {
    #[fail(display = "Invalid activation token")]
    InvalidUsernameOrPassword,

    #[fail(display = "Account is currently locked")]
    AccountLocked,

    #[fail(display = "Account is not activated")]
    AccountNotYetActivated,

    #[fail(display = "Internal error")]
    Internal,
}
