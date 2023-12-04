use rust_decimal::prelude::ToPrimitive;
use crate::items::entity::{Item as EntityItem, ItemDTO};
use crate::items::grpc_items::Item as GrpcItem;
use rust_decimal::Decimal;
use crate::items::grpc_items::{CreateItemRequest, UpdateItemRequest};
use helper::convert_to_decimal;


impl From<EntityItem> for GrpcItem {
    fn from(item: EntityItem) -> Self {
        let most_liked = item.most_liked.map(|d| d.to_f64().unwrap_or(0.0));
        let enabled_for_menu = item.enabled_for_menu.map(|i| i != 0);
        let restricted_18 = item.restricted_18.map(|i| i != 0);
  
        GrpcItem {
             id: item.id,
             name: item.name,
             description: item.description,
             max_capacity_day: item.max_capacity_day,
             most_liked: most_liked.unwrap_or(0.0),
             enabled_for_menu: enabled_for_menu.unwrap_or(false),
             is_active: item.is_active.unwrap_or(0),
             restricted_18: restricted_18.unwrap_or(false),
             estimated_minutes_item_preparation: item.estimated_minutes_item_preparation.unwrap_or(0),
             created_at: item.created_at.to_string(),
             updated_at: item.updated_at.to_string(),
        }
    }
  }
  
  pub fn map_create_item_request_to_item_dto(request: CreateItemRequest) -> ItemDTO {
    let most_liked: Option<Decimal> = Some(convert_to_decimal(request.most_liked));
    ItemDTO {
        name: request.name,
        description: request.description,
        max_capacity_day: request.max_capacity_day,
        most_liked,
        enabled_for_menu: Some(if request.enabled_for_menu { 1 } else { 0 }),
        is_active: Some(request.is_active),
        restricted_18: Some(if request.restricted_18 { 1 } else { 0 }),
        estimated_minutes_item_preparation: Some(request.estimated_minutes_item_preparation),
       // created_at: None,
        //updated_at: None,
    }
  }
  
  pub fn map_update_item_request_to_item_dto(request: UpdateItemRequest) -> ItemDTO {
    ItemDTO {
        name: request.name,
        description: request.description,
        max_capacity_day: request.max_capacity_day,
        most_liked: Some(
            request.most_liked.to_string().parse().unwrap_or_else(|_| rust_decimal::Decimal::ZERO)
        ),
        enabled_for_menu: Some(if request.enabled_for_menu { 1 } else { 0 }),
        is_active: Some(request.is_active),
        restricted_18: Some(if request.restricted_18 { 1 } else { 0 }),
        estimated_minutes_item_preparation: Some(request.estimated_minutes_item_preparation),
        //created_at: None, 
        //updated_at: None, 
    }
  }