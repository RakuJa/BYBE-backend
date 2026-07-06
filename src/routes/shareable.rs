use actix_web::error::ErrorBadRequest;
use actix_web::{Responder, Result, get, post, web};
use bybe::models::shearable_data::ShareableNpcList;
use bybe::models::shearable_data::ShareableShop;
use bybe::models::shearable_data::{LegacyShareableEncounter, ShareableEncounter};
use bybe::traits::base64::base64_decode::Base64Decode;
use bybe::traits::base64::base64_encode::Base64Encode;
use serde::Serialize;
use utoipa::OpenApi;

pub fn init_endpoints(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shareable")
            .service(get_shop_shareable_link)
            .service(get_npc_shareable_link)
            .service(get_encounter_shareable_link)
            .service(get_legacy_encounter_shareable_link)
            .service(get_shop_from_shareable_link)
            .service(get_npc_list_from_shareable_link)
            .service(get_encounter_from_shareable_link)
            .service(get_legacy_encounter_from_shareable_link),
    );
}

pub fn init_docs() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            get_shop_shareable_link,
            get_npc_shareable_link,
            get_encounter_shareable_link,
            get_legacy_encounter_shareable_link,
            get_shop_from_shareable_link,
            get_npc_list_from_shareable_link,
            get_encounter_from_shareable_link,
            get_legacy_encounter_from_shareable_link
        ),
        components(schemas(
            ShareableNpcList,
            ShareableShop,
            LegacyShareableEncounter,
            ShareableEncounter,
        ))
    )]
    struct ApiDoc;
    ApiDoc::openapi()
}

async fn encode_response<T: Base64Encode + Sync>(body: T, err_msg: &'static str) -> Result<String> {
    body.encode().await.map_err(|_| ErrorBadRequest(err_msg))
}

async fn decode_response<T: Base64Decode + Serialize + Send>(
    encoded: String,
    err_msg: &'static str,
) -> Result<web::Json<T>> {
    T::decode(encoded)
        .await
        .map(web::Json)
        .map_err(|_| ErrorBadRequest(err_msg))
}

#[utoipa::path(
    post,
    path = "/shareable/shop/encode",
    tags = ["shop", "shareable"],
    request_body(
        content = ShareableShop,
        description = "Get unique link for given shop data",
        content_type = "application/json",
    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/shop/encode")]
pub async fn get_shop_shareable_link(
    web::Json(body): web::Json<ShareableShop>,
) -> Result<impl Responder> {
    encode_response(body, "Invalid JSON data for Shop").await
}

#[utoipa::path(
    post,
    path = "/shareable/npc/encode",
    tags = ["npc", "shareable"],
    request_body(
        content = ShareableNpcList,
        description = "Get unique link for given npc list data",
        content_type = "application/json",
    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/npc/encode")]
pub async fn get_npc_shareable_link(
    web::Json(body): web::Json<ShareableNpcList>,
) -> Result<impl Responder> {
    encode_response(body, "Invalid JSON data for Npc").await
}

#[utoipa::path(
    post,
    path = "/shareable/encounter/legacy/encode",
    tags = ["encounter", "shareable"],
    request_body(
        content = LegacyShareableEncounter,
        description = "Get unique link for given encounter data",
        content_type = "application/json",
    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/encounter/legacy/encode")]
pub async fn get_legacy_encounter_shareable_link(
    web::Json(body): web::Json<LegacyShareableEncounter>,
) -> Result<impl Responder> {
    encode_response(body, "Invalid JSON data for Encounter").await
}

#[utoipa::path(
    post,
    path = "/shareable/encounter/encode",
    tags = ["encounter", "shareable"],
    request_body(
        content = ShareableEncounter,
        description = "Get unique link for given encounter data",
        content_type = "application/json",
    ),
    responses(
        (status=200, description = "Successful Response", body = String),
        (status=400, description = "Bad request.")
    ),
)]
#[post("/encounter/encode")]
pub async fn get_encounter_shareable_link(
    web::Json(body): web::Json<ShareableEncounter>,
) -> Result<impl Responder> {
    encode_response(body, "Invalid JSON data for Encounter").await
}

#[utoipa::path(
    get,
    path = "/shareable/shop/decode/{encoded_data}",
    tags = ["shop", "shareable"],
    responses(
        (status=200, description = "Successful Response", body = [ShareableShop]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/shop/decode/{encoded_data}")]
pub async fn get_shop_from_shareable_link(
    encoded_data: web::Path<String>,
) -> Result<impl Responder> {
    decode_response::<ShareableShop>(encoded_data.into_inner(), "Invalid link for shop").await
}

#[utoipa::path(
    get,
    path = "/shareable/npc/decode/{encoded_data}",
    tags = ["npc", "shareable"],
    responses(
        (status=200, description = "Successful Response", body = [ShareableNpcList]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/npc/decode/{encoded_data}")]
pub async fn get_npc_list_from_shareable_link(
    encoded_data: web::Path<String>,
) -> Result<impl Responder> {
    decode_response::<ShareableNpcList>(encoded_data.into_inner(), "Invalid link for npc list")
        .await
}

#[utoipa::path(
    get,
    path = "/shareable/encounter/decode/legacy/{encoded_data}",
    tags = ["encounter", "shareable"],
    responses(
        (status=200, description = "Successful Response", body = [LegacyShareableEncounter]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/encounter/decode/legacy/{encoded_data}")]
pub async fn get_legacy_encounter_from_shareable_link(
    encoded_data: web::Path<String>,
) -> Result<impl Responder> {
    decode_response::<LegacyShareableEncounter>(
        encoded_data.into_inner(),
        "Invalid link for encounter",
    )
    .await
}

#[utoipa::path(
    get,
    path = "/shareable/encounter/decode/{encoded_data}",
    tags = ["encounter", "shareable"],
    responses(
        (status=200, description = "Successful Response", body = [ShareableEncounter]),
        (status=400, description = "Bad request.")
    ),
)]
#[get("/encounter/decode/{encoded_data}")]
pub async fn get_encounter_from_shareable_link(
    encoded_data: web::Path<String>,
) -> Result<impl Responder> {
    decode_response::<ShareableEncounter>(encoded_data.into_inner(), "Invalid link for encounter")
        .await
}
