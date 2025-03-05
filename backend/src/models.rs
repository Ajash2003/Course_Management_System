use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
//use crate::schema::courses;

#[derive(Queryable, Serialize, Debug)]
pub struct Course {
    pub id: i32,
    pub title: String,
    pub code: String,
    pub credits: i32,
    pub department: String,
    pub description: Option<String>, 
}

#[derive(Deserialize)]
pub struct CreditsFilter {
    pub credits: i32,
}

#[derive(Serialize)]
pub struct DeleteResult {
    pub deleted_count: usize,
    pub deleted_courses: Vec<Course>, 
}
