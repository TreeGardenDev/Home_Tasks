use crate::utils;
use crate::models;
use diesel::prelude::*;
use std::time::SystemTime;

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

