use diesel::table;
table! {
    app_user (id) {
        id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        version -> Int4,
        first_name -> Text,
        last_name -> Text,
        username -> Text,
        email -> Text,
        phone_number -> Nullable<Text>,
        activated -> Bool,
        locked -> Bool,
        failed_logins -> Int2,
    }
}

table! {
    country (id) {
        id -> Int4,
        code -> Text,
        name -> Text,
        dial_code -> Int2,
        currency -> Text,
    }
}

table! {
    currency (id) {
        id -> Int4,
        code -> Text,
        symbol -> Nullable<Text>,
        name -> Text,
        precision -> Int2,
        format -> Text,
    }
}

table! {
    date_format (id) {
        id -> Int4,
        c_format -> Text,
        date_picker_format -> Text,
        js_format -> Text,
    }
}

table! {
    datetime_format (id) {
        id -> Int4,
        c_format -> Text,
        js_format -> Text,
    }
}

table! {
    language (id) {
        id -> Int4,
        name -> Text,
        locale -> Text,
    }
}

table! {
    message (id) {
        id -> Int8,
        created_at -> Timestamptz,
        subject -> Text,
        message_type -> Text,
        body_type -> Text,
        body -> Text,
    }
}

table! {
    message_address (id) {
        id -> Int8,
        message_id -> Int8,
        address_type -> Text,
        name -> Nullable<Text>,
        address -> Text,
    }
}

table! {
    message_attachment (id) {
        id -> Int8,
        message_id -> Int8,
        name -> Text,
        data -> Bytea,
    }
}

table! {
    onetime_token (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        token_type -> Text,
        token -> Text,
        created_at -> Timestamptz,
        expiry_date -> Timestamptz,
    }
}



table! {
    timezone (id) {
        id -> Int4,
        name -> Text,
        gmt_offset -> Text,
        location -> Text,
    }
}

table! {
    user_password (user_id) {
        user_id -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        hash -> Text,
        expiry_date -> Timestamptz,
    }
}

joinable!(message_address -> message (message_id));
joinable!(message_attachment -> message (message_id));
joinable!(onetime_token -> app_user (user_id));
joinable!(user_password -> app_user (user_id));

allow_tables_to_appear_in_same_query!(
    app_user,
    country,
    currency,
    date_format,
    datetime_format,
    language,
    message,
    message_address,
    message_attachment,
    onetime_token,
    timezone,
    user_password,
);
