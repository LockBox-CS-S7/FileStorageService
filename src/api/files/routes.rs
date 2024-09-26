use actix_web::{web, Scope};
use super::handlers;

pub fn routes() -> Scope {
    web::scope("/files")
}