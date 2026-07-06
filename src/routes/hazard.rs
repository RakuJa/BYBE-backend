use crate::sanitizer::sanitize_id;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use bybe::AppState;
use bybe::models::hazard::hazard_field_filter::HazardFieldFilters;
use bybe::models::hazard::hazard_listing_struct::{
    HazardListingPaginatedRequest, HazardListingSortData,
};
use bybe::models::hazard::hazard_struct::HazardRanges;
use bybe::models::response_data::{HazardListingResponse, ResponseHazard};
use bybe::models::routers_validator_structs::PaginatedRequest;
use bybe::models::shared::action::Action;
use bybe::models::shared::game_system_enum::GameSystem;
use bybe::services::hazard_service;
use utoipa::OpenApi;

macro_rules! define_hazard {
    ($prefix:ident, $system:expr, $tag:literal) => {
        paste::paste! {
            pub fn [<$prefix _init_endpoints>](cfg: &mut web::ServiceConfig) {
                cfg.service(
                    web::scope("/hazard")
                        .service([<$prefix _get_hazard_listing>])
                        .service([<$prefix _get_hazard_traits_list>])
                        .service([<$prefix _get_hazard_sources_list>])
                        .service([<$prefix _get_hazard_rarities_list>])
                        .service([<$prefix _get_hazard_sizes_list>])
                        .service([<$prefix _get_hazard_ranges>])
                        .service([<$prefix _get_hazard>]), // last, to avoid wildcard matching on source/traits/etc
                );
            }

            pub fn [<$prefix _init_docs>]() -> utoipa::openapi::OpenApi {
                #[derive(OpenApi)]
                #[openapi(
                    paths(
                        [<$prefix _get_hazard_listing>],
                        [<$prefix _get_hazard_traits_list>],
                        [<$prefix _get_hazard_sources_list>],
                        [<$prefix _get_hazard_rarities_list>],
                        [<$prefix _get_hazard_sizes_list>],
                        [<$prefix _get_hazard_ranges>],
                        [<$prefix _get_hazard>],
                    ),
                    components(schemas(
                        HazardFieldFilters,
                        HazardListingSortData,
                        HazardListingResponse,
                        Action,
                        HazardRanges,
                    ))
                )]
                struct ApiDoc;
                ApiDoc::openapi()
            }

            #[utoipa::path(
                post,
                path = "/hazard/list",
                tags = [$tag, "hazard"],
                request_body(content = HazardFieldFilters, content_type = "application/json"),
                params(PaginatedRequest, HazardListingSortData),
                responses(
                    (status=200, description = "Successful Response", body = HazardListingResponse),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/list")]
            pub async fn [<$prefix _get_hazard_listing>](
                data: web::Data<AppState>,
                web::Json(body): web::Json<HazardFieldFilters>,
                pagination: Query<PaginatedRequest>,
                sort_data: Query<HazardListingSortData>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    hazard_service::get_hazard_listing(
                        &data,
                        &body,
                        &HazardListingPaginatedRequest {
                            paginated_request: pagination.0,
                            hazard_sort_data: sort_data.0,
                        },
                        $system,
                    )
                    .await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/hazard/traits",
                tags = [$tag, "hazard"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/traits")]
            pub async fn [<$prefix _get_hazard_traits_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    hazard_service::get_traits_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/hazard/sources",
                tags = [$tag, "hazard"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/sources")]
            pub async fn [<$prefix _get_hazard_sources_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    hazard_service::get_sources_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/hazard/rarities",
                tags = [$tag, "hazard"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/rarities")]
            pub async fn [<$prefix _get_hazard_rarities_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    hazard_service::get_rarities_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/hazard/sizes",
                tags = [$tag, "hazard"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/sizes")]
            pub async fn [<$prefix _get_hazard_sizes_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    hazard_service::get_sizes_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/hazard/ranges",
                tags = [$tag, "hazard"],
                responses(
                    (status=200, description = "Successful Response", body = HazardRanges),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/ranges")]
            pub async fn [<$prefix _get_hazard_ranges>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    hazard_service::get_hazard_ranges(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/hazard/{hazard_id}",
                tags = [$tag, "hazard"],
                params(
                    ("hazard_id" = String, Path, description = "id of the hazard to fetch"),
                ),
                responses(
                    (status=200, description = "Successful Response", body = ResponseHazard),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/{hazard_id}")]
            pub async fn [<$prefix _get_hazard>](
                data: web::Data<AppState>,
                hazard_id: web::Path<String>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    hazard_service::get_hazard(&data, sanitize_id(&hazard_id)?, $system).await,
                ))
            }
        }
    };
}

define_hazard!(pf, GameSystem::Pathfinder, "pf");
define_hazard!(sf, GameSystem::Starfinder, "sf");
