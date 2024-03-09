use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

impl QuestionDetail {
    pub fn new(question: Question) -> Self {
        Self {
            question_uuid: Uuid::new_v4().to_string(),
            title: question.title,
            description: question.description,
            created_at: Utc::now().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct QuestionId{
    pub question_uuid: String
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

impl AnswerDetail {
    pub fn new(answer: Answer) -> Self {
        Self {
            answer_uuid: Uuid::new_v4().to_string(),
            question_uuid: answer.question_uuid,
            content: answer.content,
            created_at: Utc::now().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AnswerId{
    pub answer_uuid: String
}