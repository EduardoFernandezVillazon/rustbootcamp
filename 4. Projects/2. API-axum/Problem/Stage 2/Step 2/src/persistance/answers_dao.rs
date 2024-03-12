use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::error::DatabaseError;
use uuid::Uuid;
use time::{PrimitiveDateTime, OffsetDateTime};

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswersDaoImpl { 
            db: db 
        }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {

        let uuid = Uuid::parse_str(&answer.question_uuid).map_err(|e| DBError::InvalidUUID(answer.question_uuid.clone()))?;

        let now_odt = OffsetDateTime::now_utc();
        let now_pdt = PrimitiveDateTime::new(now_odt.date(), now_odt.time());
        let record = sqlx::query!(
            "INSERT INTO answers ( question_uuid, content, answer_uuid, created_at )
            VALUES ( $1, $2, $3, $4 )
            RETURNING *", uuid, answer.content, Uuid::new_v4(), now_pdt)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e{
                sqlx::Error::Database(ref f) => match f.code() {
                    Some(std::borrow::Cow::Borrowed(postgres_error_codes::FOREIGN_KEY_VIOLATION)) => DBError::InvalidUUID(answer.question_uuid.clone()),
                    _ => DBError::Other(Box::new(e)),
                },
                _ => DBError::Other(Box::new(e))
            }
            )?;


        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {

        let uuid = Uuid::parse_str(&answer_uuid).map_err(|e| DBError::InvalidUUID(answer_uuid))?;

        let query = sqlx::query!("
        DELETE FROM answers WHERE answer_uuid = $1
        RETURNING answer_uuid", uuid)
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        info!("Answer with uuid {:?} successfully deleted", query);

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {

        let uuid = Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(question_uuid))?;

        let records = sqlx::query!("
        SELECT * FROM answers WHERE question_uuid = $1", uuid)
        .fetch_all(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        let answers = records.iter().map(|record| AnswerDetail{
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content.clone(),
            created_at: record.created_at.to_string(),
        }).collect::<Vec<AnswerDetail>>();

        Ok(answers)
    }
}
