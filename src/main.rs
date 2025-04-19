mod models;
mod utils;
mod db;
use diesel::prelude::*;
use std::time::SystemTime;


fn main() {
    let mut _con = utils::establish_connection();   
    //let method=String::from("create_list");
    let method=String::from("create_item");
    //let mut new_list = models::NewList::new();
    if method=="create_list"{
        let current_time = SystemTime::now();
        let new_list = db::build_list("Test List".to_string(), current_time, current_time);
        //let insert_list = db::create_list(&new_list);
        let _= db::create_list(&new_list);
    }
    if method=="create_item"{
        let current_time = SystemTime::now();
        let new_item = db::build_item("Test List".to_string(),"Test Item 4".to_string(), current_time);
        let _= db::create_item(&new_item);
        if new_item.list_id.is_some(){
            db::increase_item_count(new_item.list_id.unwrap());
        }
        
    }
    
    let listitem= db::ListItem::new("Test List".to_string());
    
    println!("{:?}", listitem);
}
