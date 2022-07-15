// table! {
//     folders (id) {
//         id -> Uuid,
//         name -> Varchar,
//         parent -> Nullable<Uuid>,
//         words -> Array<Uuid>,
//         created_at -> Timestamptz,
//         updated_at -> Timestamptz,
//     }
// }

// table! {
//     quiz_results (id) {
//         id -> Uuid,
//         results -> Array<crate::quiz::QuizResult>,
//         created_at -> Timestamptz,
//     }
// }

// table! {
//     use diesel::sql_types::*;
//     use crate::psql::models::PgWordTypeMapping;
//     words (id) {
//         id -> Uuid,
//         word -> Varchar,
//         kind -> PgWordTypeMapping,
//         tags -> Array<Text>,
//         translations -> Array<crate::psql::models::PgTranslationEntry>,
//         created_at -> Timestamptz,
//         updated_at -> Timestamptz,
//     }
// }

// allow_tables_to_appear_in_same_query!(folders, quiz_results, words,);
