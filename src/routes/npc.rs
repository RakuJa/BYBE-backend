use actix_web::error::ErrorBadRequest;
use actix_web::{Responder, get, post, web};
use bybe::AppState;
use bybe::db::json_fetcher;
use bybe::models::npc::ancestry_enum::{PfAncestry, SfAncestry};
use bybe::models::npc::class_enum::{PfClass, SfClass};
use bybe::models::npc::culture_enum::PfCulture;
use bybe::models::npc::gender_enum::Gender;
use bybe::models::npc::job_enum::{PfJob, SfJob};
use bybe::models::npc::name_origin_enum::{
    PfNameOrigin, PfNameOriginFilter, SfNameOrigin, SfNameOriginFilter,
};
use bybe::models::npc::request_npc_struct::{AncestryData, RandomNameData, RandomNpcData};
use bybe::models::response_data::ResponseNpc;
use bybe::models::routers_validator_structs::LevelData;
use bybe::models::shared::game_system_enum::GameSystem;
use bybe::services::npc_service;
use utoipa::OpenApi;

macro_rules! define_npc_handlers {
    ($prefix:ident, $system:expr, $tag:literal, $class:ty, $ancestry:ty, $job:ty, $name_origin:ty, $name_filter:ty) => {
        paste::paste! {
            #[utoipa::path(
                post,
                path = "/npc/generator",
                tags = [$tag, "npc"],
                request_body(content = RandomNpcData<$class, $name_filter, $job>, content_type = "application/json"),
                responses(
                    (status=200, description = "Successful Response", body = ResponseNpc),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator")]
            pub async fn [<$prefix _get_random_npc>](
                data: web::Data<AppState>,
                body: Option<web::Json<RandomNpcData<$class, $name_filter, $job>>>,
            ) -> actix_web::Result<impl Responder> {
                let npc_data = body.map(|x| x.0).unwrap_or_default();
                if npc_data.is_valid() {
                    npc_service::generate_random_npc(&data, npc_data).map_or_else(
                        |_| Err(ErrorBadRequest(
                            "Given parameters are not valid. Check for conflicts e.g. Ancestry's unsupported gender chosen",
                        )),
                        |npc| Ok(web::Json(npc)),
                    )
                } else {
                    Err(ErrorBadRequest(
                        "Given parameters are not valid. Check for conflicts e.g. Ancestry's unsupported gender chosen",
                    ))
                }
            }

            #[utoipa::path(
                post,
                path = "/npc/generator/class",
                tags = [$tag, "npc"],
                request_body(content = Option<Vec<$class>>, content_type = "application/json"),
                responses(
                    (status=200, description = "Successful Response", body = String),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator/class")]
            pub async fn [<$prefix _get_random_class>](
                body: Option<web::Json<Vec<$class>>>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(npc_service::get_random_class(
                    body.map(|b| b.0).unwrap_or_default(),
                )))
            }

            #[utoipa::path(
                post,
                path = "/npc/generator/level",
                tags = [$tag, "npc"],
                request_body(content = LevelData, content_type = "application/json"),
                responses(
                    (status=200, description = "Successful Response", body = i64),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator/level")]
            pub async fn [<$prefix _get_random_level>](
                body: Option<web::Json<LevelData>>,
            ) -> actix_web::Result<impl Responder> {
                if let Some(json) = &body
                    && !json.0.is_data_valid()
                {
                    return Err(ErrorBadRequest(
                        "Given parameters are not valid. Check for conflicts e.g. min lvl > max lvl",
                    ));
                }
                Ok(web::Json(npc_service::get_random_level(body.map(|x| x.0))))
            }

            #[utoipa::path(
                post,
                path = "/npc/generator/ancestry",
                tags = [$tag, "npc"],
                request_body(content = Option<Vec<$ancestry>>, content_type = "application/json"),
                responses(
                    (status=200, description = "Successful Response", body = String),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator/ancestry")]
            pub async fn [<$prefix _get_random_ancestry>](
                body: Option<web::Json<Vec<$ancestry>>>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(npc_service::get_random_ancestry(body.map(|b| b.0))))
            }

            #[utoipa::path(
                post,
                path = "/npc/generator/gender",
                tags = [$tag, "npc"],
                request_body(content = Option<Vec<Gender>>, content_type = "application/json"),
                responses(
                    (status=200, description = "Successful Response", body = String),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator/gender")]
            pub async fn [<$prefix _get_random_gender>](
                body: Option<web::Json<Vec<Gender>>>,
            ) -> actix_web::Result<impl Responder> {
                npc_service::get_random_gender(body.map(|b| b.0)).map_or_else(
                    |_| Err(ErrorBadRequest(
                        "Given parameters are not valid. Check for empty whitelist vector (if whitelist is empty there cannot be a valid gender)",
                    )),
                    |g| Ok(web::Json(g)),
                )
            }

            #[utoipa::path(
                post,
                path = "/npc/generator/job",
                tags = [$tag, "npc"],
                request_body(content = Option<Vec<$job>>, content_type = "application/json"),
                responses(
                    (status=200, description = "Successful Response", body = String),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator/job")]
            pub async fn [<$prefix _get_random_job>](
                body: Option<web::Json<Vec<$job>>>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(npc_service::get_random_job(
                    body.map(|b| b.0).unwrap_or_default(),
                )))
            }

            #[utoipa::path(
                post,
                path = "/npc/generator/nickname",
                tags = [$tag, "npc"],
                responses(
                    (status=200, description = "Successful Response", body = String),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator/nickname")]
            pub async fn [<$prefix _get_random_nickname>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(npc_service::generate_random_nickname(
                    &json_fetcher::get_nicknames(&data),
                )))
            }

            #[utoipa::path(
                post,
                path = "/npc/generator/names",
                tags = [$tag, "npc"],
                request_body(content = RandomNameData<$name_origin>, content_type = "application/json"),
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator/names")]
            pub async fn [<$prefix _get_random_names>](
                data: web::Data<AppState>,
                body: Option<web::Json<RandomNameData<$name_origin>>>,
            ) -> actix_web::Result<impl Responder> {
                if let Some(json) = body {
                    let rd = json.0;
                    if rd.is_valid() {
                        Ok(web::Json(npc_service::generate_random_names(
                            rd,
                            &json_fetcher::get_names(&data),
                        )))
                    } else {
                        Err(ErrorBadRequest(
                            "Given parameters are not valid. Check for conflicts e.g. Ancestry unsupported gender chosen",
                        ))
                    }
                } else {
                    Ok(web::Json(npc_service::generate_random_names(
                        RandomNameData::default_with_system(<$name_origin>::default()),
                        &json_fetcher::get_names(&data),
                    )))
                }
            }

            #[utoipa::path(
                get,
                path = "/npc/ancestries",
                tags = [$tag, "npc"],
                responses(
                    (status=200, description = "Successful Response", body = [AncestryData]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/ancestries")]
            pub async fn [<$prefix _get_npc_ancestries_list>]() -> actix_web::Result<impl Responder> {
                Ok(web::Json(npc_service::get_ancestries_list($system)))
            }

            #[utoipa::path(
                get,
                path = "/npc/cultures",
                tags = [$tag, "npc"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/cultures")]
            pub async fn [<$prefix _get_npc_cultures_list>]() -> actix_web::Result<impl Responder> {
                Ok(web::Json(npc_service::get_cultures_list()))
            }

            #[utoipa::path(
                get,
                path = "/npc/genders",
                tags = [$tag, "npc"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/genders")]
            pub async fn [<$prefix _get_npc_genders_list>]() -> actix_web::Result<impl Responder> {
                Ok(web::Json(npc_service::get_genders_list()))
            }

            #[utoipa::path(
                get,
                path = "/npc/jobs",
                tags = [$tag, "npc"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/jobs")]
            pub async fn [<$prefix _get_npc_jobs_list>]() -> actix_web::Result<impl Responder> {
                Ok(web::Json(npc_service::get_jobs_list($system)))
            }

            #[utoipa::path(
                get,
                path = "/npc/classes",
                tags = [$tag, "npc"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/classes")]
            pub async fn [<$prefix _get_npc_classes_list>]() -> actix_web::Result<impl Responder> {
                Ok(web::Json(npc_service::get_classes_list($system)))
            }
        }
    };
}

macro_rules! define_npc {
    // With culture generator endpoint (PF)
    ($prefix:ident, $system:expr, $tag:literal, $class:ty, $ancestry:ty, $job:ty, $name_origin:ty, $name_filter:ty, culture: $culture:ty) => {
        paste::paste! {
            pub fn [<$prefix _init_endpoints>](cfg: &mut web::ServiceConfig) {
                cfg.service(
                    web::scope("/npc")
                        .service([<$prefix _get_random_npc>])
                        .service([<$prefix _get_random_ancestry>])
                        .service([<$prefix _get_random_culture>])
                        .service([<$prefix _get_random_class>])
                        .service([<$prefix _get_random_gender>])
                        .service([<$prefix _get_random_job>])
                        .service([<$prefix _get_random_names>])
                        .service([<$prefix _get_random_nickname>])
                        .service([<$prefix _get_random_level>])
                        .service([<$prefix _get_npc_classes_list>])
                        .service([<$prefix _get_npc_genders_list>])
                        .service([<$prefix _get_npc_jobs_list>])
                        .service([<$prefix _get_npc_ancestries_list>])
                        .service([<$prefix _get_npc_cultures_list>]),
                );
            }

            pub fn [<$prefix _init_docs>]() -> utoipa::openapi::OpenApi {
                #[derive(OpenApi)]
                #[openapi(
                    paths(
                        [<$prefix _get_random_npc>],
                        [<$prefix _get_random_ancestry>],
                        [<$prefix _get_random_culture>],
                        [<$prefix _get_random_class>],
                        [<$prefix _get_random_gender>],
                        [<$prefix _get_random_job>],
                        [<$prefix _get_random_names>],
                        [<$prefix _get_random_nickname>],
                        [<$prefix _get_random_level>],
                        [<$prefix _get_npc_classes_list>],
                        [<$prefix _get_npc_genders_list>],
                        [<$prefix _get_npc_jobs_list>],
                        [<$prefix _get_npc_ancestries_list>],
                        [<$prefix _get_npc_cultures_list>],
                    ),
                    components(schemas(
                        ResponseNpc,
                        RandomNpcData<$class, $name_filter, $job>,
                        RandomNameData<$name_origin>,
                        AncestryData,
                    ))
                )]
                struct ApiDoc;
                ApiDoc::openapi()
            }

            #[utoipa::path(
                post,
                path = "/npc/generator/culture",
                tags = [$tag, "npc"],
                request_body(content = Option<Vec<$culture>>, content_type = "application/json"),
                responses(
                    (status=200, description = "Successful Response", body = String),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator/culture")]
            pub async fn [<$prefix _get_random_culture>](
                body: Option<web::Json<Vec<$culture>>>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(npc_service::get_random_culture(body.map(|b| b.0))))
            }
        }
        define_npc_handlers!(
            $prefix,
            $system,
            $tag,
            $class,
            $ancestry,
            $job,
            $name_origin,
            $name_filter
        );
    };

    // Without culture generator endpoint (SF)
    ($prefix:ident, $system:expr, $tag:literal, $class:ty, $ancestry:ty, $job:ty, $name_origin:ty, $name_filter:ty) => {
        paste::paste! {
            pub fn [<$prefix _init_endpoints>](cfg: &mut web::ServiceConfig) {
                cfg.service(
                    web::scope("/npc")
                        .service([<$prefix _get_random_npc>])
                        .service([<$prefix _get_random_ancestry>])
                        .service([<$prefix _get_random_class>])
                        .service([<$prefix _get_random_gender>])
                        .service([<$prefix _get_random_job>])
                        .service([<$prefix _get_random_names>])
                        .service([<$prefix _get_random_nickname>])
                        .service([<$prefix _get_random_level>])
                        .service([<$prefix _get_npc_classes_list>])
                        .service([<$prefix _get_npc_genders_list>])
                        .service([<$prefix _get_npc_jobs_list>])
                        .service([<$prefix _get_npc_ancestries_list>])
                        .service([<$prefix _get_npc_cultures_list>]),
                );
            }

            pub fn [<$prefix _init_docs>]() -> utoipa::openapi::OpenApi {
                #[derive(OpenApi)]
                #[openapi(
                    paths(
                        [<$prefix _get_random_npc>],
                        [<$prefix _get_random_ancestry>],
                        [<$prefix _get_random_class>],
                        [<$prefix _get_random_gender>],
                        [<$prefix _get_random_job>],
                        [<$prefix _get_random_names>],
                        [<$prefix _get_random_nickname>],
                        [<$prefix _get_random_level>],
                        [<$prefix _get_npc_classes_list>],
                        [<$prefix _get_npc_genders_list>],
                        [<$prefix _get_npc_jobs_list>],
                        [<$prefix _get_npc_ancestries_list>],
                        [<$prefix _get_npc_cultures_list>],
                    ),
                    components(schemas(
                        ResponseNpc,
                        RandomNpcData<$class, $name_filter, $job>,
                        RandomNameData<$name_origin>,
                        AncestryData,
                    ))
                )]
                struct ApiDoc;
                ApiDoc::openapi()
            }
        }
        define_npc_handlers!(
            $prefix,
            $system,
            $tag,
            $class,
            $ancestry,
            $job,
            $name_origin,
            $name_filter
        );
    };
}

define_npc!(
    pf, GameSystem::Pathfinder, "pf",
    PfClass, PfAncestry, PfJob, PfNameOrigin, PfNameOriginFilter, culture: PfCulture
);

define_npc!(
    sf,
    GameSystem::Starfinder,
    "sf",
    SfClass,
    SfAncestry,
    SfJob,
    SfNameOrigin,
    SfNameOriginFilter
);
