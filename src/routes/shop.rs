use bybe::models::item::armor_struct::ArmorData;
use bybe::models::item::item_field_filter::ItemFieldFilters;
use bybe::models::item::item_metadata::type_enum::{ItemTypeEnum, WeaponTypeEnum};
use bybe::models::item::item_struct::Item;
use bybe::models::item::shield_struct::ShieldData;
use bybe::models::item::shop_structs::{
    ItemSortEnum, PfShopTemplateEnum, RandomShopData, SfShopTemplateEnum, ShopPaginatedRequest,
    ShopRanges, ShopSortData, ShopTemplateData,
};
use bybe::models::item::weapon_struct::{DamageData, WeaponData};
use bybe::models::response_data::{ResponseItem, ShopListingResponse};
use bybe::models::routers_validator_structs::{Dice, PaginatedRequest};
use bybe::models::shared::game_system_enum::GameSystem;
use bybe::services::shop_service;
use bybe::AppState;
use actix_web::web::Query;
use actix_web::{Responder, get, post, web};
use crate::sanitizer::sanitize_id;
use utoipa::OpenApi;

macro_rules! define_shop {
    ($prefix:ident, $system:expr, $tag:literal, $template_enum:ty) => {
        paste::paste! {
            pub fn [<$prefix _init_endpoints>](cfg: &mut web::ServiceConfig) {
                cfg.service(
                    web::scope("/shop")
                        .service([<$prefix _get_item>])
                        .service([<$prefix _get_shop_listing>])
                        .service([<$prefix _get_items_traits_list>])
                        .service([<$prefix _get_templates_data>])
                        .service([<$prefix _get_items_sources_list>])
                        .service([<$prefix _get_shop_ranges>])
                        .service([<$prefix _get_random_shop_listing>]),
                );
            }

            pub fn [<$prefix _init_docs>]() -> utoipa::openapi::OpenApi {
                #[derive(OpenApi)]
                #[openapi(
                    paths(
                        [<$prefix _get_shop_listing>],
                        [<$prefix _get_item>],
                        [<$prefix _get_random_shop_listing>],
                        [<$prefix _get_items_traits_list>],
                        [<$prefix _get_templates_data>],
                        [<$prefix _get_items_sources_list>],
                        [<$prefix _get_shop_ranges>],
                    ),
                    components(schemas(
                        ResponseItem,
                        ItemTypeEnum,
                        ShopListingResponse,
                        Item,
                        RandomShopData<$template_enum>,
                        Dice,
                        $template_enum,
                        ShopTemplateData,
                        ItemFieldFilters,
                        ItemSortEnum,
                        DamageData,
                        WeaponData,
                        ArmorData,
                        ShieldData,
                        WeaponTypeEnum,
                        ShopRanges,
                    ))
                )]
                struct ApiDoc;
                ApiDoc::openapi()
            }

            #[utoipa::path(
                post,
                path = "/shop/list",
                tags = [$tag, "shop"],
                request_body(content = ItemFieldFilters, content_type = "application/json"),
                params(PaginatedRequest, ShopSortData),
                responses(
                    (status=200, description = "Successful Response", body = ShopListingResponse),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/list")]
            pub async fn [<$prefix _get_shop_listing>](
                data: web::Data<AppState>,
                web::Json(body): web::Json<ItemFieldFilters>,
                pagination: Query<PaginatedRequest>,
                sort_data: Query<ShopSortData>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    shop_service::get_shop_listing(
                        &data,
                        &body,
                        &ShopPaginatedRequest {
                            paginated_request: pagination.0,
                            shop_sort_data: sort_data.0,
                        },
                        $system,
                    )
                    .await,
                ))
            }

            #[utoipa::path(
                post,
                path = "/shop/generator",
                tags = [$tag, "shop"],
                request_body(content = RandomShopData<$template_enum>, content_type = "application/json"),
                responses(
                    (status=200, description = "Successful Response", body = ShopListingResponse),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[post("/generator")]
            pub async fn [<$prefix _get_random_shop_listing>](
                data: web::Data<AppState>,
                web::Json(body): web::Json<RandomShopData<$template_enum>>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    shop_service::generate_random_shop_listing(&data, &body, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/shop/item/{item_id}",
                tags = [$tag, "shop"],
                params(("item_id" = String, Path, description = "id of the item to fetch")),
                responses(
                    (status=200, description = "Successful Response", body = ResponseItem),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/item/{item_id}")]
            pub async fn [<$prefix _get_item>](
                data: web::Data<AppState>,
                item_id: web::Path<String>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    shop_service::get_item(&data, sanitize_id(&item_id)?, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/shop/sources",
                tags = [$tag, "shop"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/sources")]
            pub async fn [<$prefix _get_items_sources_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    shop_service::get_sources_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/shop/traits",
                tags = [$tag, "shop"],
                responses(
                    (status=200, description = "Successful Response", body = [String]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/traits")]
            pub async fn [<$prefix _get_items_traits_list>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    shop_service::get_traits_list(&data, $system).await,
                ))
            }

            #[utoipa::path(
                get,
                path = "/shop/templates_data",
                tags = [$tag, "shop"],
                responses(
                    (status=200, description = "Successful Response", body = [ShopTemplateData]),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/templates_data")]
            pub async fn [<$prefix _get_templates_data>]() -> actix_web::Result<impl Responder> {
                Ok(web::Json(shop_service::get_shop_templates_data($system)))
            }

            #[utoipa::path(
                get,
                path = "/shop/ranges",
                tags = [$tag, "shop"],
                responses(
                    (status=200, description = "Successful Response", body = ShopRanges),
                    (status=400, description = "Bad request.")
                ),
            )]
            #[get("/ranges")]
            pub async fn [<$prefix _get_shop_ranges>](
                data: web::Data<AppState>,
            ) -> actix_web::Result<impl Responder> {
                Ok(web::Json(
                    shop_service::get_shop_ranges(&data, $system).await,
                ))
            }
        }
    };
}

define_shop!(pf, GameSystem::Pathfinder, "pf", PfShopTemplateEnum);
define_shop!(sf, GameSystem::Starfinder, "sf", SfShopTemplateEnum);
