use async_trait::async_trait;
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use time::{PrimitiveDateTime, OffsetDateTime};
use log::{info, warn, error};

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        QuestionsDaoImpl {
            db: db,
        }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {

        let now_odt = OffsetDateTime::now_utc();
        let now_pdt = PrimitiveDateTime::new(now_odt.date(), now_odt.time());
        let record = sqlx::query!("
        INSERT INTO questions ( title, description, question_uuid, created_at )
        VALUES ( $1, $2, $3, $4 )
        RETURNING *", question.title, question.description, Uuid::new_v4(), now_pdt)
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        
        let uuid = Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(question_uuid))?;

        let query = sqlx::query!("
        DELETE FROM questions WHERE question_uuid = $1
        RETURNING question_uuid", uuid)
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        info!("Question with uuid {:?} successfully deleted", query);
        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {

        let records = sqlx::query!("
        SELECT * FROM questions")
        .fetch_all(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        let questions = records.iter().map(|record| QuestionDetail{
            question_uuid: record.question_uuid.to_string(),
            title: record.title.clone(),
            description: record.description.clone(),
            created_at: record.created_at.to_string(),
        }).collect::<Vec<QuestionDetail>>();

        Ok(questions)
    }
}
