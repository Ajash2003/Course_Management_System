// src/routes.rs
use rocket::serde::json::Json;
use rocket::serde::{Serialize};
//use rocket::http::Status;
use crate::db;
use crate::models::{CreditsFilter, DeleteResult};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

type JsonResponse<T> = Json<ApiResponse<T>>;

#[post("/courses/delete", format = "json", data = "<credits_filter>")]
pub fn delete_courses(credits_filter: Json<CreditsFilter>) -> JsonResponse<DeleteResult> {
    match db::delete_courses_by_credits(credits_filter.credits) {
        Ok(result) => {
            let message = if result.deleted_count == 0 {
                format!("No courses with {} credits found to delete", credits_filter.credits)
            } else {
                format!("Successfully deleted {} course(s) with {} credits", 
                       result.deleted_count, credits_filter.credits)
            };
            
            Json(ApiResponse {
                success: true,
                message,
                data: Some(result),
            })
        },
        Err(e) => {
            Json(ApiResponse {
                success: false,
                message: format!("Failed to delete courses: {}", e),
                data: None,
            })
        }
    }
}

#[get("/health")]
pub fn health_check() -> JsonResponse<()> {
    Json(ApiResponse {
        success: true,
        message: "API is up and running".to_string(),
        data: None,
    })
}