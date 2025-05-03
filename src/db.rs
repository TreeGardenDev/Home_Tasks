use crate::models;
use crate::utils;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Bool;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub fn build_list(
    title: String,
    datetime: SystemTime,
    duedate: SystemTime,
    parent_item_id: Option<i32>,
) -> models::NewList {
    let mut new_list = models::NewList::new();
    new_list.title = Some(title);
    new_list.created_at = Some(datetime);
    new_list.updated_at = Some(datetime);
    new_list.due_date = Some(duedate);
    new_list.parent_item_id = parent_item_id;
    new_list
}
pub fn create_list(new_list: &models::NewList) -> models::List {
    let mut _con = utils::establish_connection();
    diesel::insert_into(models::lists::table)
        .values(new_list)
        .get_result(&mut _con)
        .expect("Error saving new list")
}

pub fn build_item(
    title: String,
    name: String,
    datetime: SystemTime,
    other_child_list_id: Option<i32>,
) -> models::NewItem {
    let mut new_item = models::NewItem::new();
    new_item.name = name;
    new_item.created_at = Some(datetime);
    new_item.updated_at = Some(datetime);
    new_item.list_id = Some(get_listid_by_title(title));
    new_item.child_list_id = other_child_list_id;
    new_item
}

pub fn create_item(new_item: &models::NewItem) -> models::Item {
    let mut _con = utils::establish_connection();

    diesel::insert_into(models::items::table)
        .values(new_item)
        .get_result(&mut _con)
        .expect("Error saving new item")
}
pub fn add_child_list(itemid: i32, other_child_list_id: i32) -> models::Item {
    let mut _con = utils::establish_connection();
    let item: models::Item = models::items::table
        .filter(models::items::id.eq(itemid))
        .first(&mut _con)
        .expect("Error loading item");
    diesel::update(models::items::table.find(itemid))
        .set(models::items::child_list_id.eq(Some(other_child_list_id)))
        .execute(&mut _con)
        .expect("Error updating item");
    return item;
}

pub fn get_listid_by_title(title: String) -> i32 {
    let mut _con = utils::establish_connection();
    let list: models::List = models::lists::table
        .filter(models::lists::title.eq(title))
        .first(&mut _con)
        .expect("Error loading list");
    let id = list.id;
    return id;
}
pub fn get_list_by_title(title: String) -> models::List {
    let mut _con = utils::establish_connection();
    let list: models::List = models::lists::table
        .filter(models::lists::title.eq(title))
        .first(&mut _con)
        .expect("Error loading list");
    return list;
}
pub fn get_item_by_listid(listid: i32) -> Vec<models::Item> {
    let mut _con = utils::establish_connection();
    let items: Vec<models::Item> = models::items::table
        .filter(models::items::list_id.eq(listid))
        .load(&mut _con)
        .expect("Error loading items");
    return items;
}
pub fn get_itemlist_by_listid(listid: i32) -> ItemListVec {
    let mut _con = utils::establish_connection();
    let items: Vec<models::Item> = models::items::table
        .filter(models::items::list_id.eq(listid))
        .load(&mut _con)
        .expect("Error loading items");
    let itemvec = ItemListVec::build(items);
    //for item in items.iter() {
    //    let mut itemlist_item = ItemList::from_item(item);
    //    if item.child_list_id.is_some() {
    //        let child_list = get_list_at_id(item.child_list_id.unwrap());
    //        itemlist_item = itemlist_item.with_child_list(child_list);
    //    }
    //    itemlist.push(itemlist_item);
    //}
    return itemvec;
}
pub fn get_item_query(title: String) -> Vec<models::Item> {
    let mut _con = utils::establish_connection();
    let where_clause = format!(
        "EXISTS (SELECT 1 FROM lists WHERE lists.id = items.list_id AND lists.title = '{}')",
        title
    );
    let items: Vec<models::Item> = models::items::table
        .filter(sql::<Bool>(where_clause.as_str()))
        .load(&mut _con)
        .expect("Error loading items");
    return items;
}
pub fn increase_item_count(listid: i32) {
    let mut _con = utils::establish_connection();
    let list: models::List = models::lists::table
        .filter(models::lists::id.eq(listid))
        .first(&mut _con)
        .expect("Error loading list");
    diesel::update(models::lists::table.find(listid))
        .set(models::lists::number_of_items.eq(Some(list.number_of_items.unwrap() + 1)))
        .execute(&mut _con)
        .expect("Error updating list");
}
pub fn decrease_item_count(listid: i32) {
    let mut _con = utils::establish_connection();
    let list: models::List = models::lists::table
        .filter(models::lists::id.eq(listid))
        .first(&mut _con)
        .expect("Error loading list");
    diesel::update(models::lists::table.find(listid))
        .set(models::lists::number_of_items.eq(Some(list.number_of_items.unwrap() - 1)))
        .execute(&mut _con)
        .expect("Error updating list");
}
pub fn complete_item(itemid: i32) {
    let mut _con = utils::establish_connection();
    diesel::update(models::items::table.find(itemid))
        .set(models::items::completed.eq(true))
        .execute(&mut _con)
        .expect("Error updating item");
}
pub fn complete_multiple_items(itemids: Vec<i32>) {
    let mut _con = utils::establish_connection();
    diesel::update(models::items::table.filter(models::items::id.eq_any(itemids)))
        .set(models::items::completed.eq(true))
        .execute(&mut _con)
        .expect("Error updating items");
}
pub fn complete_list(listid: i32) {
    let mut _con = utils::establish_connection();
    let completable = check_completed_required(listid);
    if !completable {
        println!("List cannot be completed, some required items are not completed.");
        return;
    }
    diesel::update(models::lists::table.find(listid))
        .set(models::lists::completed.eq(true))
        .execute(&mut _con)
        .expect("Error updating list");
}
pub fn check_completed_required(listid: i32) -> bool {
    let mut _con = utils::establish_connection();
    let items = models::items::table
        .filter(models::items::list_id.eq(listid))
        .filter(models::items::completed.eq(false))
        .filter(models::items::required.eq(true))
        .load::<models::Item>(&mut _con)
        .expect("Error loading items");

    if items.len() > 0 {
        return false;
    }
    return true;
}
pub fn delete_item(itemid: i32) {
    let mut _con = utils::establish_connection();
    let item: models::Item = models::items::table
        .filter(models::items::id.eq(itemid))
        .first(&mut _con)
        .expect("Error loading item");
    if item.list_id.is_some() {
        decrease_item_count(item.list_id.unwrap());
    }
    diesel::delete(models::items::table.find(itemid))
        .execute(&mut _con)
        .expect("Error deleting item");
}
pub fn delete_multiple_items(itemids: Vec<i32>) {
    let mut _con = utils::establish_connection();
    let new_itemids: Vec<i32> = itemids.clone();
    for itemid in itemids {
        let item: models::Item = models::items::table
            .filter(models::items::id.eq(itemid))
            .first(&mut _con)
            .expect("Error loading item");
        if item.list_id.is_some() {
            decrease_item_count(item.list_id.unwrap());
        }
    }
    diesel::delete(models::items::table.filter(models::items::id.eq_any(new_itemids)))
        .execute(&mut _con)
        .expect("Error deleting items");
}
pub fn change_item_required(itemid: i32, required: bool) {
    let mut _con = utils::establish_connection();
    diesel::update(models::items::table.find(itemid))
        .set(models::items::required.eq(required))
        .execute(&mut _con)
        .expect("Error updating item");
}

pub fn get_all_lists() -> Vec<models::List> {
    let mut _con = utils::establish_connection();
    let lists: Vec<models::List> = models::lists::table
        .load(&mut _con)
        .expect("Error loading lists");
    return lists;
}
pub fn get_list_at_id(listid: i32) -> ListItem {
    let mut _con = utils::establish_connection();
    let list: models::List = models::lists::table
        .filter(models::lists::id.eq(listid))
        .first(&mut _con)
        .expect("Error loading list");
    let querylist = ListItem {
        list,
        items: get_itemlist_by_listid(listid),
    };
    return querylist;
}
pub fn update_item(
    itemid: i32,
    completed: bool,
    required: bool,
    delete: bool,
) -> Result<(), String> {
    if delete {
        delete_item(itemid);
        return Ok(());
    }
    let mut _con = utils::establish_connection();
    let item: models::Item = models::items::table
        .filter(models::items::id.eq(itemid))
        .first(&mut _con)
        .expect("Error loading item");

    if completed {
        complete_item(itemid);
    } else {
        diesel::update(models::items::table.find(itemid))
            .set(models::items::completed.eq(completed))
            .execute(&mut _con)
            .expect("Error updating item");
    }

    if required {
        change_item_required(itemid, required);
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem {
    pub list: models::List,
    pub items: ItemListVec,
}
impl ListItem {
    pub fn new(title: String) -> ListItem {
        let tit2 = title.clone();
        //let items:Vec<models::Item> = get_item_by_listid(get_listid_by_title(title));
        let items: Vec<models::Item> = get_item_query(title);
        let itemlistvec = ItemListVec::build(items);
        let list = get_list_by_title(tit2);

        ListItem {
            list,
            items: itemlistvec,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemList {
    //include all fields from models::Item
    pub id: i32,
    pub name: String,
    pub list_id: Option<i32>,
    pub child_list_id: Option<i32>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub completed: bool,
    pub required: bool,
    pub parent_item_id: Option<i32>,
    pub child_list: Option<ListItem>,
}
impl ItemList {
    pub fn new() -> ItemList {
        ItemList {
            id: 0,
            name: String::new(),
            list_id: None,
            child_list_id: None,
            created_at: None,
            updated_at: None,
            completed: false,
            required: false,
            parent_item_id: None,
            child_list: None,
        }
    }
    pub fn from_item(item: &models::Item) -> ItemList {
        ItemList {
            id: item.id,
            name: item.name.clone(),
            list_id: item.list_id,
            child_list_id: item.child_list_id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            completed: item.completed.unwrap_or(false),
            required: item.required.unwrap_or(false),
            parent_item_id: Some(item.id),
            child_list: None, // This will be set later if needed
        }
    }
    pub fn with_child_list(mut self, child_list: ListItem) -> Self {
        self.child_list = Some(child_list);
        self
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemListVec {
    pub items: Vec<ItemList>,
}
impl ItemListVec {
    pub fn new() -> ItemListVec {
        ItemListVec { items: Vec::new() }
    }
    pub fn build(items: Vec<models::Item>) -> ItemListVec {
        let mut itemlist = ItemListVec::new();
        for item in items.iter() {
            let mut itemlist_item = ItemList::from_item(item);
            if item.child_list_id.is_some() {
                let child_list = get_list_at_id(item.child_list_id.unwrap());
                itemlist_item = itemlist_item.with_child_list(child_list);
            }
            itemlist.items.push(itemlist_item);
        }
        itemlist
    }
}
//#[derive(Debug, Serialize, Deserialize)]
//pub struct ItemQuery {
//    pub title: String,
//    pub items: ItemListVec,
//}
//#[derive(Debug, Serialize, Deserialize)]
//pub struct BulkUpdate {
//    pub itemid: i32,
//    pub completed: bool,
//    pub required: bool,
//    pub delete: bool,
//}
//impl BulkUpdate {
//    pub fn new() -> BulkUpdate {
//        BulkUpdate {
//            itemid: 0,
//            completed: false,
//            required: false,
//            delete: false,
//        }
//    }
//    pub fn update_item(&mut self, itemid: i32, completed: bool, required: bool, delete: bool) {
//        self.itemid = itemid;
//        self.completed = completed;
//        self.required = required;
//        self.delete = delete;
//    }
//}
//#[derive(Debug, Serialize, Deserialize)]
//pub struct BulkUpdateTotal {
//    pub items: Vec<BulkUpdate>,
//}
//impl BulkUpdateTotal {
//    pub fn new() -> BulkUpdateTotal {
//        BulkUpdateTotal { items: Vec::new() }
//    }
//    pub fn add_item(&mut self, item: BulkUpdate) {
//        self.items.push(item);
//    }
//}
