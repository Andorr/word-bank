use crate::WordType;

table! {
    folders (id) {
        id -> Uuid,
        name -> Varchar,
        parent -> Nullable<Uuid>,
        words -> Array<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    quiz_results (id) {
        id -> Uuid,
        results -> Array<crate::quiz::QuizResult>,
        created_at -> Timestamptz,
    }
}

table! {
    words (id) {
        id -> Uuid,
        word -> Varchar,
        kind -> crate::WordType,
        tags -> Array<Text>,
        translations -> Array<crate::Translation>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    folders,
    quiz_results,
    words,
);
