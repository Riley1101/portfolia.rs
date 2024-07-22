// @generated automatically by Diesel CLI.

diesel::table! {
    article_categories (article_id, category_id) {
        article_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    articles (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        #[max_length = 255]
        thumbnail_image -> Nullable<Varchar>,
        #[max_length = 255]
        static_file -> Nullable<Varchar>,
        published -> Bool,
        createdat -> Timestamp,
        updatedat -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(article_categories -> articles (article_id));
diesel::joinable!(article_categories -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    article_categories,
    articles,
    categories,
);
