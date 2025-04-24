//use actix_web::HttpRequest;
use actix_web::{web, Result, HttpResponse};
use serde::Deserialize;
use crate::db;
//use crate::models;
//system time
//use std::time::SystemTime;
use actix_web_lab::extract::Path;


#[derive(Deserialize)]
pub struct PostList{
    title: String,
    parent_item_id: Option<i32>
}

#[derive(Deserialize)]
pub struct PostItem {
    title: String,
    name: String,
    child_list_id: Option<i32>
}
#[derive(Deserialize,Clone)]
pub struct DeleteItem {
    item_id: i32
}

pub async fn create_list(post_list: web::Json<PostList>) -> Result<String> {
    let current_time = std::time::SystemTime::now();
    let mut parent_item:Option<i32>=None;
    let parent_item_id = post_list.parent_item_id.clone();
    let mut child =false;
    if post_list.parent_item_id.is_some() {
        parent_item = post_list.parent_item_id.clone();
        child = true;
    }
    let new_list = db::build_list(post_list.title.clone(),  current_time, current_time,parent_item);
    let _ = db::create_list(&new_list);
    if child {
        let listid = db::get_listid_by_title(post_list.title.clone());
        let _ = db::add_child_list(parent_item_id.unwrap(), listid);
    }

    Ok(format!("List '{}'", post_list.title))
}

pub async fn create_item(post_item: web::Json<PostItem>) -> Result<String> {
    let current_time = std::time::SystemTime::now();
    let mut child_list_id:Option<i32> = None;
    if post_item.child_list_id.is_some() {
        child_list_id = post_item.child_list_id.clone();
    }
    let new_item = db::build_item(post_item.title.clone(), post_item.name.clone(),   current_time, child_list_id);
    let listid = db::get_listid_by_title(post_item.title.clone());
    if new_item.list_id.is_none() {
        return Ok(format!("List with title '{}' not found", post_item.title));
    }
    let _= db::increase_item_count(listid);
    let _ = db::create_item(&new_item);
    Ok(format!("Item '{}' created in list '{}'", post_item.title, post_item.name))
}

pub async fn delete_item(item: web::Json<DeleteItem>) -> Result<String> {
    let id=item.item_id.clone();
    let _ = db::delete_item(item.item_id);
    Ok(format!("Item with ID {} deleted", id))
}
pub async fn delete_multiple_items(item_ids: web::Json<Vec<DeleteItem>>) -> Result<String> {
    let ids= item_ids.clone();
    let storeid= item_ids.iter().map(|item| item.item_id).collect::<Vec<i32>>();
    let _ = db::delete_multiple_items(storeid);
    Ok(format!("Items with IDs {:?} deleted", ids.iter().map(|item| item.item_id).collect::<Vec<i32>>()))
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
pub async fn query_at_node(Path((list_id,)):Path<(i32,)>) -> HttpResponse {
    // Start query at list that has list id of list_id

    let list=db::get_list_at_id(list_id.clone());

    HttpResponse::Ok().json(list)
    
}

