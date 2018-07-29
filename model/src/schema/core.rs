use diesel::table;

table! {
    country (id) {
        id -> Int4,
        code -> Varchar,
        name -> Varchar,
        dial_code -> Int2,
        currency -> Varchar,
    }
}

table! {
    currency (id) {
        id -> Int4,
        code -> Varchar,
        symbol -> Nullable<Varchar>,
        name -> Varchar,
        precision -> Int2,
        format -> Varchar,
    }
}

table! {
    date_format (id) {
        id -> Int4,
        c_format -> Varchar,
        date_picker_format -> Varchar,
        js_format -> Varchar,
    }
}

table! {
    datetime_format (id) {
        id -> Int4,
        c_format -> Varchar,
        js_format -> Varchar,
    }
}

table! {
    language (id) {
        id -> Int4,
        name -> Varchar,
        locale -> Varchar,
    }
}

table! {
    timezone (id) {
        id -> Int4,
        name -> Varchar,
        gmt_offset -> Varchar,
        location -> Varchar,
    }
}

table! {
    app_user (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        phone_number -> Nullable<Varchar>,
        activated -> Bool,
        locked -> Bool,
        failed_logins -> Int2,
    }
}

table! {
    user_password (user_id) {
        user_id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        hash -> Varchar,
        expiry_date -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    onetime_token (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        token_type -> Varchar,
        token -> Varchar,
        created_at -> Timestamptz,
        expiry_date -> Timestamptz,
    }
}
joinable!(onetime_token -> app_user (user_id));

allow_tables_to_appear_in_same_query!(
    app_user,
    country,
    currency,
    date_format,
    datetime_format,
    language,
    timezone,
    user_password,
    onetime_token
);

table! {
    message (id) {
        id -> Int8,
        created_at -> Timestamptz,
        subject -> Varchar,
        message_type -> Varchar,
        body_type -> Varchar,
        body -> Binary,
    }
}

table! {
    message_address (id) {
        id -> Int8,
        message_id -> Int8,
        address_type -> Varchar,
        name -> Nullable<Varchar>,
        address -> Varchar,
    }
}

table! {
    message_attachment (id) {
        id -> Int8,
        message_id -> Int8,
        name -> Varchar,
        data -> Binary,
    }
}

joinable!(user_password -> app_user (user_id));
