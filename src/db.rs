use crate::utils;
use crate::models;
use diesel::prelude::*;
use std::time::SystemTime;
use diesel::dsl::{sql,sql_query};
use diesel::sql_types::Bool;


pub fn build_list(title:String,datetime:SystemTime, duedate:SystemTime) -> models::NewList {
    let mut new_list = models::NewList::new();
    new_list.title = Some(title);
    new_list.created_at = Some(datetime);
    new_list.updated_at = Some(datetime);
    new_list.due_date = Some(duedate);
    new_list
}
pub fn create_list(new_list: &models::NewList) -> models::List {
    let mut _con = utils::establish_connection();
    diesel::insert_into(models::lists::table)
        .values(new_list)
        .get_result(&mut _con)
        .expect("Error saving new list")
}


pub fn build_item(title:String,name:String, datetime:SystemTime) -> models::NewItem {
    let mut new_item = models::NewItem::new();
    new_item.name= name;
    new_item.created_at = Some(datetime);
    new_item.updated_at = Some(datetime);
    new_item.list_id = Some(get_listid_by_title(title));
    new_item
}

pub fn create_item(new_item: &models::NewItem) -> models::Item {
    let mut _con = utils::establish_connection();

    
    diesel::insert_into(models::items::table)
        .values(new_item)
        .get_result(&mut _con)
        .expect("Error saving new item")
}

pub fn get_listid_by_title(title:String) -> i32 {
    let mut _con = utils::establish_connection();
    let list:models::List=models::lists::table
        .filter(models::lists::title.eq(title))
        .first(&mut _con)
        .expect("Error loading list");
    let id = list.id;
    return id;
}
pub fn get_list_by_title(title:String) -> models::List {
    let mut _con = utils::establish_connection();
    let list:models::List=models::lists::table
        .filter(models::lists::title.eq(title))
        .first(&mut _con)
        .expect("Error loading list");
    return list;
}
pub fn get_item_by_listid(listid:i32) -> Vec<models::Item> {
    let mut _con = utils::establish_connection();
    let items:Vec<models::Item> = models::items::table
        .filter(models::items::list_id.eq(listid))
        .load(&mut _con)
        .expect("Error loading items");
    return items;
}
pub fn get_item_query(title:String) -> Vec<models::Item> {
    let mut _con = utils::establish_connection();
    let where_clause = format!("EXISTS (SELECT 1 FROM lists WHERE lists.id = items.list_id AND lists.title = '{}')", title);
    let items:Vec<models::Item> = models::items::table
        .filter(
            sql::<Bool>(where_clause.as_str())
            )
        .load(&mut _con)
        .expect("Error loading items");
    return items;
}
pub fn increase_item_count(listid:i32) {
    let mut _con = utils::establish_connection();
    let list:models::List=models::lists::table
        .filter(models::lists::id.eq(listid))
        .first(&mut _con)
        .expect("Error loading list");
    diesel::update(models::lists::table.find(listid))
        .set(models::lists::number_of_items.eq(Some(list.number_of_items.unwrap()+1)))
        .execute(&mut _con)
        .expect("Error updating list");
}

pub fn get_all_lists() -> Vec<models::List> {
    let mut _con = utils::establish_connection();
    let lists:Vec<models::List> = models::lists::table
        .load(&mut _con)
        .expect("Error loading lists");
    return lists;
}

#[derive(Debug)]
pub struct ListItem{
    pub list: models::List,
    pub items: Vec<models::Item>
}
impl ListItem {
    pub fn new(title:String) -> ListItem {
        let tit2=title.clone();
        //let items:Vec<models::Item> = get_item_by_listid(get_listid_by_title(title));
        let items:Vec<models::Item> = get_item_query(title);
        let list=get_list_by_title(tit2);
        ListItem {
            list,
            items
        }
        
    }
    
}
