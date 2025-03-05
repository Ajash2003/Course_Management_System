use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::models::{Course, DeleteResult};
use crate::schema::courses;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn delete_courses_by_credits(credit_val: i32) -> Result<DeleteResult, diesel::result::Error> { 
    let mut conn = establish_connection();
    
    let courses_to_delete = courses::table
        .filter(courses::credits.eq(credit_val))
        .load::<Course>(&mut conn)?; 
    
    let deleted_count = diesel::delete(
        courses::table.filter(courses::credits.eq(credit_val))
    ).execute(&mut conn)?;
    
    Ok(DeleteResult {
        deleted_count,
        deleted_courses: courses_to_delete,
    })
}