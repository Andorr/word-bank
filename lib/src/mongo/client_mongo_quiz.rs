use chrono::Utc;
use uuid::Uuid;

use crate::quiz::QuizResult;

use super::{models::QuizResultDBM, MongoContext, MongoDBClient};

impl MongoDBClient {
    pub fn handle_insert_quiz_result(
        &self,
        ctx: &mut MongoContext,
        result: &mut QuizResult,
    ) -> Result<Uuid, ()> {
        result.created_at = Utc::now();

        let qrdbm: QuizResultDBM = result.into();

        let col = self.quizresult_collection();

        match col.insert_one_with_session(qrdbm, None, &mut ctx.session) {
            Ok(_) => {
                println!("Result was ok!");
                Ok(result.id)
            }
            Err(err) => {
                println!("{:?}", err);
                Err(())
            }
        }
    }
}
