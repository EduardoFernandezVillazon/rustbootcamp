use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{models::*, AppState};

use std::sync::Arc;

mod handlers_inner;

impl IntoResponse for handlers_inner::HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

// ---- CRUD for Questions ----

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    // NOTE: `IntoResponse` is implemented for `HandlerError` above
    let question_created = handlers_inner::create_question(question, questions_dao).await;
    match question_created{
        Ok(question) => Ok(Json(QuestionDetail {
            question_uuid: format!("question_uuid: {}", question.question_uuid).to_owned(),
            title: format!("title: {}", question.title).to_owned(),
            description: format!("description: {}", question.description).to_owned(),
            created_at: format!("created_at: {}", question.created_at).to_owned(),
        })),
        Err(err) => Err(err.into_response()),
    }
    



}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // TODO: Replace the fake data below with a call to `handlers_inner::read_questions`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    let questions = handlers_inner::read_questions(questions_dao).await;
    match questions {
        Ok(questions) => Ok(Json(questions)),
        Err(err) => Err(err.into_response()),
    }
    //
    //Ok(Json(vec![QuestionDetail {
     //   question_uuid: "question_uuid".to_owned(),
     //   title: "title".to_owned(),
     //   description: "description".to_owned(),
     //   created_at: "created_at".to_owned(),
    //}]))
    //
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Make a call to `handlers_inner::delete_question`.
    // Return a unit type in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    let question_deleted = handlers_inner::delete_question(question_uuid, questions_dao).await;
    match question_deleted {
        Ok(()) => Ok(()),
        Err(err) => Err(err.into_response()),
    }
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> Result<impl IntoResponse, impl IntoResponse> {

    // TODO: Replace the fake data below with a call to `handlers_inner::create_answer`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    let answer_created = handlers_inner::create_answer(answer, answers_dao).await;
    match answer_created {
        Ok(answer) => Ok(Json(answer)),
        Err(err) => Err(err.into_response()),
    }

}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // TODO: Replace the fake data below with a call to `handlers_inner::read_answers`.
    // Return the result wrapped in JSON in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    let answers = handlers_inner::read_answers(question_uuid, answers_dao).await;
    match answers {
        Ok(answers) => Ok(Json(answers)),
        Err(err) => Err(err.into_response()),
    }
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // TODO: update return type to be of type `Result`. Both the Ok and Err case should contain `impl IntoResponse`.
    // TODO: Make a call to `handlers_inner::delete_answer`.
    // Return a unit type in the success case and an `HandlerError` in the error case.
    // NOTE: `IntoResponse` is implemented for `HandlerError` above.
    let answer_deleted = handlers_inner::delete_answer(answer_uuid, answers_dao).await;
    match answer_deleted {
        Ok(()) => Ok(()),
        Err(err) => Err(err.into_response()),
    }
}
