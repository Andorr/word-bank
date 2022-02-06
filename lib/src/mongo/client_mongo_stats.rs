use std::collections::HashMap;

use bson::doc;

use crate::{client::Context, models::stats::UserStatistics};

use super::MongoDBClient;

impl MongoDBClient {
    pub fn handle_get_user_statistics(&self, ctx: &mut Context) -> Result<UserStatistics, ()> {
        // Get word statistics
        let word_kind_result = self.word_collection().aggregate_with_session(
            vec![doc! {
                "$group": doc!{
                    "_id": "kind",
                    "count": {"$count": { } }
                 }
            }],
            None,
            &mut ctx.session,
        );
        if word_kind_result.is_err() {
            return Err(());
        }
        let word_count_by_kind: HashMap<String, u64> = word_kind_result
            .unwrap()
            .iter(&mut ctx.session)
            .filter(|f| f.is_ok())
            .map(|f| f.unwrap())
            .map(|f| {
                (
                    f.get_str("_id").unwrap_or("UNKNOWN").to_string(),
                    f.get_i64("count").unwrap() as u64,
                )
            })
            .collect();

        // Get number of quizes
        let quiz_count = match self.quizresult_collection().count_documents_with_session(
            doc! {},
            None,
            &mut ctx.session,
        ) {
            Ok(count) => count,
            Err(_) => return Err(()),
        };

        Ok(UserStatistics {
            word_count: *(&word_count_by_kind.values().sum::<u64>()),
            word_types: word_count_by_kind,
            quiz_count,
        })
    }
}
