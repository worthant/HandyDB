use actix_web::{web, HttpResponse, Responder};
use crate::db::KvStore;
use std::sync::Arc;

pub async fn get(
    kv_store: web::Data<Arc<KvStore>>,
    key: web::Path<String>,
) -> impl Responder {
    match kv_store.get(&key) {
        Some(value) => HttpResponse::Ok().body(value),
        None => HttpResponse::NotFound().body("Key not found"),
    }
}
