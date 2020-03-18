table! {
    comments (id) {
        id -> Integer,
        user_id -> Integer,
        exercise_id -> Integer,
        text -> Text,
        created -> Timestamp,
    }
}

table! {
    deals (id) {
        id -> Integer,
        dealer -> Text,
        vulnerable -> Text,
        north -> Text,
        east -> Text,
        south -> Text,
        west -> Text,
    }
}

table! {
    exercise_bids (id) {
        id -> Integer,
        created -> Timestamp,
        exercise_id -> Integer,
        user_id -> Integer,
        bid -> Text,
    }
}

table! {
    exercises (id) {
        id -> Integer,
        deal_id -> Integer,
        bids -> Text,
        parent_id -> Nullable<Integer>,
        created -> Timestamp,
    }
}

table! {
    tokens (token) {
        token -> Text,
        user_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Text,
        pw_hash -> Text,
        last_active -> Timestamp,
    }
}

joinable!(comments -> exercises (exercise_id));
joinable!(comments -> users (user_id));
joinable!(exercise_bids -> exercises (exercise_id));
joinable!(exercise_bids -> users (user_id));
joinable!(exercises -> deals (deal_id));
joinable!(tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(comments, deals, exercise_bids, exercises, tokens, users,);
