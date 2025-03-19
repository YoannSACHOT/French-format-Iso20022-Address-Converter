use actix_web::{
    delete, get, post, put, web::{self, Json, Path}, HttpResponse,
    Responder,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::models::{AddressKind, FrenchAddressBuilder};
use crate::domain::usecases::{convert_to_french, convert_to_iso};
use crate::infrastructure::app_state::AppState;

#[derive(Deserialize)]
pub struct FrenchAddressPayload {
    pub kind: String,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub line3: Option<String>,
    pub line4: Option<String>,
    pub line5: Option<String>,
    pub line6: Option<String>,
    pub line7: Option<String>,
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list_addresses)
        .service(get_address)
        .service(add_address)
        .service(update_address)
        .service(delete_address)
        .service(convert_address);
}

#[get("/addresses")]
async fn list_addresses(data: web::Data<AppState>) -> impl Responder {
    let query_service = data.query_service.lock().unwrap();
    let iso_addresses = query_service.get_all_addresses();
    HttpResponse::Ok().json(iso_addresses)
}

#[get("/addresses/{id}")]
async fn get_address(data: web::Data<AppState>, path: Path<String>) -> impl Responder {
    let address_id = path.into_inner();
    let query_service = data.query_service.lock().unwrap();

    match query_service.get_address(&address_id) {
        Some(iso_address) => HttpResponse::Ok().json(iso_address),
        None => HttpResponse::NotFound().body(format!("Address {} not found", address_id)),
    }
}

#[get("/addresses/{id}/convert")]
async fn convert_address(data: web::Data<AppState>, path: Path<String>) -> impl Responder {
    let address_id = path.into_inner();
    let query_service = data.query_service.lock().unwrap();

    match query_service.get_address(&address_id) {
        Some(iso_address) => {
            let french = convert_to_french(&iso_address);
            HttpResponse::Ok().json(french)
        }
        None => HttpResponse::NotFound().body(format!("Address {} not found", address_id)),
    }
}

#[post("/addresses")]
async fn add_address(
    data: web::Data<AppState>,
    payload: Json<FrenchAddressPayload>,
) -> impl Responder {
    let mut command_service = data.command_service.lock().unwrap();
    let id = Uuid::new_v4().to_string();

    // Vérifier la validité du "kind" (company / particular)
    let kind = match parse_kind(&payload.kind) {
        Ok(k) => k,
        Err(e) => return e, // renvoie un HttpResponse d'erreur
    };

    // Construire l'adresse FrenchAddress
    let french_address = match FrenchAddressBuilder::new()
        .id(id.clone())
        .line1(payload.line1.clone())
        .line2(payload.line2.clone())
        .line3(payload.line3.clone())
        .line4(payload.line4.clone())
        .line5(payload.line5.clone())
        .line6(payload.line6.clone())
        .line7(payload.line7.clone())
        .build()
    {
        Ok(addr) => addr,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    // Convertir en ISO20022Address
    let iso_address = match convert_to_iso(&french_address, kind) {
        iso => iso,
    };

    // Enregistrer via le CommandService
    if let Err(e) = command_service.add_address(iso_address) {
        return HttpResponse::InternalServerError().body(e);
    }

    HttpResponse::Created().body(format!("Address created with ID {}", id))
}

#[put("/addresses/{id}")]
async fn update_address(
    data: web::Data<AppState>,
    path: Path<String>,
    payload: Json<FrenchAddressPayload>,
) -> impl Responder {
    let id = path.into_inner();

    let query_service = data.query_service.lock().unwrap();
    let mut command_service = data.command_service.lock().unwrap();

    let kind = match parse_kind(&payload.kind) {
        Ok(k) => k,
        Err(e) => return e,
    };

    let existing_iso = match query_service.get_address(&id) {
        Some(addr) => addr,
        None => return HttpResponse::NotFound().body(format!("Address {} not found", id)),
    };

    let existing_french = convert_to_french(&existing_iso);

    let updated_french = match FrenchAddressBuilder::new()
        .id(id.clone())
        .line1(payload.line1.clone().or(existing_french.line1))
        .line2(payload.line2.clone().or(existing_french.line2))
        .line3(payload.line3.clone().or(existing_french.line3))
        .line4(payload.line4.clone().or(existing_french.line4))
        .line5(payload.line5.clone().or(existing_french.line5))
        .line6(payload.line6.clone().or(existing_french.line6))
        .line7(payload.line7.clone().or(existing_french.line7))
        .build()
    {
        Ok(addr) => addr,
        Err(e) => return HttpResponse::BadRequest().body(e),
    };

    let updated_iso = convert_to_iso(&updated_french, kind);

    if let Err(e) = command_service.update_address(updated_iso) {
        return HttpResponse::InternalServerError().body(e);
    }

    HttpResponse::Ok().body(format!("Address {} updated", id))
}

#[delete("/addresses/{id}")]
async fn delete_address(data: web::Data<AppState>, path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    let mut command_service = data.command_service.lock().unwrap();

    match command_service.delete_address(&id) {
        Ok(_) => HttpResponse::Ok().body(format!("Address {} deleted", id)),
        Err(e) => HttpResponse::NotFound().body(e),
    }
}

fn parse_kind(s: &str) -> Result<AddressKind, HttpResponse> {
    match s.to_lowercase().as_str() {
        "company" => Ok(AddressKind::Company),
        "particular" => Ok(AddressKind::Particular),
        invalid => Err(HttpResponse::BadRequest().body(format!(
            "Invalid 'kind': {}, must be 'company' or 'particular'",
            invalid
        ))),
    }
}
