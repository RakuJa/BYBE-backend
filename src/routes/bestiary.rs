use crate::sanitizer::sanitize_id;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use bybe::AppState;
use bybe::models::bestiary_structs::{
    BestiaryPaginatedRequest, BestiaryRanges, BestiarySortData, CreatureSortEnum,
};
use bybe::models::creature::creature_component::creature_combat::{
    CreatureCombatData, SavingThrows,
};
use bybe::models::creature::creature_component::creature_core::{
    CreatureCoreData, DerivedData, EssentialData,
};
use bybe::models::creature::creature_component::creature_extra::{
    AbilityScores, CreatureExtraData,
};
use bybe::models::creature::creature_component::creature_spellcaster::CreatureSpellcasterData;
use bybe::models::creature::creature_component::creature_variant::CreatureVariantData;
use bybe::models::creature::creature_field_filter::CreatureFieldFilters;
use bybe::models::creature::creature_metadata::creature_role::CreatureRoleEnum;
use bybe::models::creature::creature_metadata::type_enum::CreatureTypeEnum;
use bybe::models::creature::creature_metadata::variant_enum::CreatureVariant;
use bybe::models::creature::items::skill::Skill;
use bybe::models::creature::items::spell::Spell;
use bybe::models::creature::items::spellcaster_entry::SpellcasterEntry;
use bybe::models::db::sense::Sense;
use bybe::models::item::armor_struct::Armor;
use bybe::models::item::shield_struct::Shield;
use bybe::models::item::weapon_struct::Weapon;
use bybe::models::response_data::{
    BestiaryResponse, CreatureResponseDataModifiers, ResponseCreature,
};
use bybe::models::routers_validator_structs::{OrderEnum, PaginatedRequest};
use bybe::models::shared::action::CoreAction;
use bybe::models::shared::alignment_enum::AlignmentEnum;
use bybe::models::shared::game_system_enum::GameSystem;
use bybe::models::shared::pf_version_enum::GameSystemVersionEnum;
use bybe::models::shared::rarity_enum::RarityEnum;
use bybe::models::shared::size_enum::SizeEnum;
use bybe::services::bestiary_service;
use utoipa::OpenApi;

macro_rules! define_bestiary {
    ($prefix:ident, $system:expr, $tag:literal) => {
        paste::paste! {
            pub fn [<$prefix _init_endpoints>](cfg: &mut web::ServiceConfig) {
                cfg.service(
                    web::scope("/bestiary")
                        .service([<$prefix _get_bestiary_listing>])
                        .service([<$prefix _get_elite_creature>])
                        .service([<$prefix _get_weak_creature>])
                        .service([<$prefix _get_creature>])
                        .service([<$prefix _get_families_list>])
                        .service([<$prefix _get_traits_list>])
                        .service([<$prefix _get_sources_list>])
                        .service([<$prefix _get_rarities_list>])
                        .service([<$prefix _get_creature_types_list>])
                        .service([<$prefix _get_creature_roles_list>])
                        .service([<$prefix _get_sizes_list>])
                        .service([<$prefix _get_alignments_list>])
                        .service([<$prefix _get_bestiary_ranges>]),
                );
            }

            pub fn [<$prefix _init_docs>]() -> utoipa::openapi::OpenApi {
                #[derive(OpenApi)]
                #[openapi(
                    paths(
                        [<$prefix _get_bestiary_listing>],
                        [<$prefix _get_families_list>],
                        [<$prefix _get_traits_list>],
                        [<$prefix _get_sources_list>],
                        [<$prefix _get_rarities_list>],
                        [<$prefix _get_sizes_list>],
                        [<$prefix _get_alignments_list>],
                        [<$prefix _get_creature_types_list>],
                        [<$prefix _get_creature_roles_list>],
                        [<$prefix _get_creature>],
                        [<$prefix _get_elite_creature>],
                        [<$prefix _get_weak_creature>],
                        [<$prefix _get_bestiary_ranges>],
                    ),
                    components(schemas(
                        BestiaryResponse,
                        ResponseCreature,
                        AlignmentEnum,
                        RarityEnum,
                        SizeEnum,
                        CreatureTypeEnum,
                        CreatureVariant,
                        CreatureCoreData,
                        EssentialData,
                        DerivedData,
                        CreatureVariantData,
                        CreatureExtraData,
                        CreatureCombatData,
                        CreatureSpellcasterData,
                        Sense,
                        Spell,
                        Shield,
                        Weapon,
                        Armor,
                        SavingThrows,
                        AbilityScores,
                        CoreAction,
                        Skill,
                        CreatureRoleEnum,
                        SpellcasterEntry,
                        GameSystemVersionEnum,
                        OrderEnum,
                        CreatureSortEnum,
                        BestiaryRanges,
                    ))
                )]
                struct ApiDoc;
                ApiDoc::openapi()
            }

            #[utoipa::path(
                post,
                path = "/bestiary/list",
                tags = [$tag, "bestiary"],
                request_body(content = CreatureFieldFilters, content_type = "application/json"),
                params(PaginatedRequest, BestiarySortData),
                responses(
                    (status=200, description = "Successful Response", body = BestiaryResponse),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/list")]
            pub async fn [<$prefix _get_bestiary_listing>](
                data: web::Data<AppState>,
                web::Json(body): web::Json<CreatureFieldFilters>,
                pagination: Query<PaginatedRequest>,
                sort_data: Query<BestiarySortData>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_bestiary_listing(
                        &data,
                        &body,
                        &BestiaryPaginatedRequest {
                            paginated_request: pagination.0,
                            bestiary_sort_data: sort_data.0,
                        },
                        $system,
                    )
                    .await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/families",
                tags = [$tag, "bestiary"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/families")]
            pub async fn [<$prefix _get_families_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_families_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/traits",
                tags = [$tag, "bestiary"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/traits")]
            pub async fn [<$prefix _get_traits_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_traits_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/sources",
                tags = [$tag, "bestiary"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/sources")]
            pub async fn [<$prefix _get_sources_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_sources_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/rarities",
                tags = [$tag, "bestiary"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/rarities")]
            pub async fn [<$prefix _get_rarities_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_rarities_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/sizes",
                tags = [$tag, "bestiary"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/sizes")]
            pub async fn [<$prefix _get_sizes_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_sizes_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/alignments",
                tags = [$tag, "bestiary"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/alignments")]
            pub async fn [<$prefix _get_alignments_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_alignments_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/creature_types",
                tags = [$tag, "bestiary"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/creature_types")]
            pub async fn [<$prefix _get_creature_types_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_creature_types_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/creature_roles",
                tags = [$tag, "bestiary"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/creature_roles")]
            pub async fn [<$prefix _get_creature_roles_list>]() -> actix_web::Result<impl Responder> {
                Ok(web::Json(bestiary_service::get_creature_roles_list()))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/base/{creature_id}",
                tags = [$tag, "bestiary"],
                params(
                    ("creature_id" = String, Path, description = "id of the creature to fetch"),
                    CreatureResponseDataModifiers,
                ),
                responses(
                    (status=200, description = "Successful Response", body = ResponseCreature),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/base/{creature_id}")]
            pub async fn [<$prefix _get_creature>](
                data: web::Data<AppState>,
                creature_id: web::Path<String>,
                response_data_mods: Query<CreatureResponseDataModifiers>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_creature(
                        &data,
                        sanitize_id(&creature_id)?,
                        &response_data_mods.0,
                        $system,
                    )
                    .await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/elite/{creature_id}",
                tags = [$tag, "bestiary"],
                params(
                    ("creature_id" = String, Path, description = "id of the creature to fetch"),
                    CreatureResponseDataModifiers,
                ),
                responses(
                    (status=200, description = "Successful Response", body = ResponseCreature),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/elite/{creature_id}")]
            pub async fn [<$prefix _get_elite_creature>](
                data: web::Data<AppState>,
                creature_id: web::Path<String>,
                response_data_mods: Query<CreatureResponseDataModifiers>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_elite_creature(
                        &data,
                        sanitize_id(&creature_id)?,
                        &response_data_mods.0,
                        $system,
                    )
                    .await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/weak/{creature_id}",
                tags = [$tag, "bestiary"],
                params(
                    ("creature_id" = String, Path, description = "id of the creature to fetch"),
                    CreatureResponseDataModifiers,
                ),
                responses(
                    (status=200, description = "Successful Response", body = ResponseCreature),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/weak/{creature_id}")]
            pub async fn [<$prefix _get_weak_creature>](
                data: web::Data<AppState>,
                creature_id: web::Path<String>,
                response_data_mods: Query<CreatureResponseDataModifiers>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_weak_creature(
                        &data,
                        sanitize_id(&creature_id)?,
                        &response_data_mods.0,
                        $system,
                    )
                    .await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/bestiary/ranges",
                tags = [$tag, "bestiary"],
                responses(
                    (status=200, description = "Successful Response", body = BestiaryRanges),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/ranges")]
            pub async fn [<$prefix _get_bestiary_ranges>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    bestiary_service::get_bestiary_ranges(&data, $system).await,
                ))
            }
        }
    };
}

define_bestiary!(pf, GameSystem::Pathfinder, "pf");
define_bestiary!(sf, GameSystem::Starfinder, "sf");
