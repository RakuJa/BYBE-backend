use bybe::models::encounter_structs::{
    AdventureGroupEnum, EncounterChallengeEnum, EncounterParams, RandomCreatureData,
    RandomEncounterData, RandomHazardData,
};
use bybe::models::response_data::{EncounterInfoResponse, RandomEncounterGeneratorResponse};
use bybe::models::shared::game_system_enum::GameSystem;
use bybe::services::encounter_handler::encounter_calculator;
use bybe::services::encounter_service;
use bybe::AppState;
use actix_web::{Responder, Result, post, web};
use utoipa::OpenApi;

macro_rules! define_encounter {
    ($prefix:ident, $system:expr, $tag:literal) => {
        paste::paste! {
            pub fn [<$prefix _init_endpoints>](cfg: &mut web::ServiceConfig) {
                cfg.service(
                    web::scope("/encounter")
                        .service([<$prefix _get_encounter_info>])
                        .service([<$prefix _get_generated_random_encounter>]),
                );
            }

            pub fn [<$prefix _init_docs>]() -> utoipa::openapi::OpenApi {
                #[derive(OpenApi)]
                #[openapi(
                    paths(
                        [<$prefix _get_encounter_info>],
                        [<$prefix _get_generated_random_encounter>],
                    ),
                    components(schemas(
                        EncounterInfoResponse,
                        RandomEncounterData,
                        EncounterParams,
                        EncounterChallengeEnum,
                        AdventureGroupEnum,
                        RandomEncounterGeneratorResponse,
                        RandomCreatureData,
                        RandomHazardData,
                    ))
                )]
                struct ApiDoc;
                ApiDoc::openapi()
            }

            #[utoipa::path(
                post,
                path = "/encounter/info",
                tags = [$tag, "encounter"],
                request_body(
                    content = EncounterParams,
                    description = "Party and enemy levels. Could send one value for each, representing the average",
                    content_type = "application/json",
                ),
                responses(
                    (status=200, description = "Successful Response", body = EncounterInfoResponse),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/info")]
            pub async fn [<$prefix _get_encounter_info>](
                web::Json(body): web::Json<EncounterParams>,
            ) -> Result<impl Responder> {
                Ok(web::Json(encounter_calculator::get_encounter_info(&body)))
            }

            #[utoipa::path(
                post,
                path = "/encounter/generator",
                tags = [$tag, "encounter"],
                request_body(
                    content = RandomEncounterData,
                    description = "Party levels as a vector of integers, if min and max are not set they will not be considered. If only one of them is set, the other one will be set at the same value.",
                    content_type = "application/json",
                ),
                responses(
                    (status=200, description = "Successful Response", body = RandomEncounterGeneratorResponse),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator")]
            pub async fn [<$prefix _get_generated_random_encounter>](
                data: web::Data<AppState>,
                web::Json(body): web::Json<RandomEncounterData>,
            ) -> Result<impl Responder> {
                Ok(web::Json(
                    encounter_service::generate_random_encounter(&data, body, $system).await,
                ))
            }
        }
    };
}

define_encounter!(pf, GameSystem::Pathfinder, "pf");
define_encounter!(sf, GameSystem::Starfinder, "sf");
