use actix_web::{web, HttpResponse, Responder};
use crate::db::KvStore;
use std::sync::{Arc, Mutex};

pub async fn get(
    kv_store: web::Data<Arc<Mutex<KvStore>>>,
    key: web::Path<String>,
) -> impl Responder {
    let kv_store = kv_store.lock().unwrap();
    match kv_store.get(&key) {
        Some(value) => HttpResponse::Ok().body(value),
        None => HttpResponse::NotFound().body("Key not found"),
    }
}
