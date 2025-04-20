use actix_web::{web, App, HttpServer, Result};
use serde::Deserialize;
use crate::db;
//system time
use std::time::SystemTime;


#[derive(Deserialize)]
pub struct PostList{
    title: String,
}

#[derive(Deserialize)]
pub struct PostItem {
    title: String,
    name: String
}

pub async fn create_list(post_list: web::Json<PostList>) -> Result<String> {
    let current_time = std::time::SystemTime::now();
    let new_list = db::build_list(post_list.title.clone(),  current_time, current_time);
    let _ = db::create_list(&new_list);
    // Here you would typically insert the list into the database
    Ok(format!("List '{}'", post_list.title))
}

pub async fn create_item(post_item: web::Json<PostItem>) -> Result<String> {
    let current_time = std::time::SystemTime::now();
    let new_item = db::build_item(post_item.title.clone(), post_item.name.clone(),   current_time);
    let listid = db::get_listid_by_title(post_item.title.clone());
    if new_item.list_id.is_none() {
        return Ok(format!("List with title '{}' not found", post_item.title));
    }
    let _= db::increase_item_count(listid);
    let _ = db::create_item(&new_item);
    // Here you would typically insert the item into the database
    Ok(format!("Item '{}' created in list '{}'", post_item.title, post_item.name))
}

pub async fn delete_item(item_id: web::Path<i32>) -> Result<String> {
    let id=item_id.clone();
    let _ = db::delete_item(item_id.into_inner());
    Ok(format!("Item with ID {} deleted", id))
}
pub async fn delete_multiple_items(item_ids: web::Json<Vec<i32>>) -> Result<String> {
    let ids= item_ids.clone();
    let _ = db::delete_multiple_items(item_ids.into_inner());
    Ok(format!("Items with IDs {:?} deleted", ids))
}
pub async fn complete_item(item_id: web::Path<i32>) -> Result<String> {
    let id = item_id.clone();
    let _ = db::complete_item(item_id.into_inner());
    Ok(format!("Item with ID {} completed", id))
}
pub async fn complete_list(list_id: web::Path<i32>) -> Result<String> {
    let id= list_id.clone();
    let _ = db::complete_list(list_id.into_inner());
    Ok(format!("List with ID {} completed", id))
}

