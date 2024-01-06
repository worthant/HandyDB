use actix_web::{web, HttpResponse, Responder};
use crate::db::KvStore;
use std::sync::{Arc, Mutex};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SetRequest {
    key: String,
    value: String,
}

pub async fn set(
    kv_store: web::Data<Arc<Mutex<KvStore>>>,
    set_request: web::Json<SetRequest>,
) -> impl Responder {
    let kv_store = kv_store.lock().unwrap();
    let key = set_request.key.clone();
    let value = set_request.value.clone();
    kv_store.set(key, value);
    HttpResponse::Ok().body("Value set successfully")
}
