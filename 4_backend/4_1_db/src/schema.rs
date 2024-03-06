// @generated automatically by Diesel CLI.

diesel::table! {
    roles (slug) {
        slug -> Varchar,
        description -> Nullable<Varchar>,
        #[max_length = 1]
        perms -> Bpchar,
    }
}

diesel::table! {
    user_roles (user, role) {
        user -> Uuid,
        role -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        age -> Int4,
    }
}

diesel::joinable!(user_roles -> roles (role));
diesel::joinable!(user_roles -> users (user));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    user_roles,
    users,
);
