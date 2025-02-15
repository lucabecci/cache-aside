use actix_web::{post, get, web, HttpResponse, Responder};
use crate::web::dto::event_dto::EventDto;

#[post("/events")]
async fn create_event(
    event_dto: web::Json<EventDto>
) -> impl Responder {
    let event_dto = event_dto.into_inner();
    HttpResponse::Ok().body(event_dto.id)
}

#[get("/events")]
async fn get_event() -> impl Responder {
    HttpResponse::Ok().body(vec![])
}