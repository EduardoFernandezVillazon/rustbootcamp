use crate::models::*;
use axum::{response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    let response = QuestionDetail::new(question);
    todo!("Write question to database");
    Json(response)
}

pub async fn read_questions() -> impl IntoResponse {
    todo!("Read questions from database");
    let question = QuestionDetail{
        question_uuid: Uuid::new_v4().to_string(), // Random UUID created from, should come from the database
        title: "Question".to_string(), // Arbitrary name used, should come from the database
        description: "Question Read from database".to_string(), // Arbitrary description used, should come from the database
        created_at: Utc::now().to_string(), // Current time used, should come from the database
    };
    let questions: Vec<QuestionDetail> = vec![question];
    Json(questions)
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    todo!("Delete question from database");
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above

pub async fn create_answer(Json(answer): Json<Answer>) -> Json<AnswerDetail> {
    let response = AnswerDetail::new(answer);
    todo!("Write answer to database");
    Json(response)
}

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above

pub async fn read_answers (Json(question_uuid): Json<QuestionId>) -> Json<Vec<AnswerDetail>> {
    todo!("Read answers from database");
    let answer = AnswerDetail{
        answer_uuid: Uuid::new_v4().to_string(), // Random UUID created from, should come from the database
        question_uuid: question_uuid.question_uuid.to_string(),
        content: "Answer".to_string(), // Arbitraty content used, should come from the database
        created_at: Utc::now().to_string(), // Current time used, should come from the database
    };
    let answers: Vec<AnswerDetail> = vec![answer];
    Json(answers)
}

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above

pub async fn delete_answer (Json(answer_uuid): Json<AnswerId>) {
    todo!("Delete answer from database")
}
