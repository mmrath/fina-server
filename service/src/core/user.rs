use chrono::Duration;
use chrono::Utc;
use failure::Error;
use model::core::NewOnetimeToken;
use model::core::{
    NewUser, NewUserPassword, OnetimeToken, TokenType, User, UserPassword, UserSignup,
};
use util::{argon2_hash, argon2_verify, new_uuid, sha512, Context};

pub(crate) const SECRET_KEY: &str = "71ade6e0-51b1-4aa3-aa70-682ea7566d3f";
pub(crate) const PASSWORD_EXPIRY_DAYS: i64 = 365 * 25;
pub(crate) const USER_ACTIVATION_TOKEN_EXPIRY: i64 = 24;

pub fn create_user(context: &Context, new_user: &NewUser) -> Result<User, Error> {
    User::insert(context.db(), new_user)
}

pub fn sign_up(context: &Context, user_ac: &UserSignup) -> Result<User, Error> {
    let conn = context.db();
    debug!("User to register {:?}", user_ac);

    let new_user = NewUser {
        first_name: user_ac.first_name.as_ref(),
        last_name: user_ac.last_name.as_ref(),
        username: user_ac.email.as_ref(),
        email: user_ac.email.as_ref(),
    };

    if User::exists_by_username(conn, user_ac.email.as_ref())? {
        Err(format_err!(
            "User with email {} already exist in system",
            &user_ac.email
        ))?
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

pub fn activate(context: &Context, token: &str) -> Result<(), Error> {
    let conn = context.db();
    let (ott, mut user) = OnetimeToken::find_user_and_token(conn, token)?
        .ok_or_else(|| format_err!("Invalid token"))?; //User does not exists

    user.activated = true;
    User::activate(conn, &user)?;
    OnetimeToken::delete(conn, ott.id)
}

fn send_activation_email(user: &User, token: &str) {
    info!("User:{:?}, activation code: {:?}", user, token);
}

pub fn login(context: &Context, username: &str, password: &str) -> Result<User, Error> {
    let conn = context.db();

    let mut user =
        User::find_by_username(conn, username)?.ok_or_else(|| format_err!("User not found"))?;
    let up = UserPassword::find(conn, user.id)?;
    let password_sha512 = sha512(password.as_ref());
    let valid = argon2_verify(&password_sha512, SECRET_KEY.as_ref(), &up.hash)?;

    if !valid {
        user.failed_logins += 1;
        let _ = User::update(conn, &user)?;
        bail!("Invalid username or password")
    } else if !user.activated {
        bail!("Please activate your account before logging in")
    } else if user.locked {
        bail!("Account is currently locked. Please reset your password.")
    }

    Ok(user)
}

pub fn find_by_id(context: &Context, id: i64) -> Result<Option<User>, Error> {
    let conn = context.db();
    User::find(conn, id)
}
