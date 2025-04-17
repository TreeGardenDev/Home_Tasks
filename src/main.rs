mod models;
mod utils;
mod db;
use diesel::prelude::*;
use std::time::SystemTime;


fn main() {
    let mut _con = utils::establish_connection();   
    let method=String::from("create");
    //let mut new_list = models::NewList::new();
    if method=="create"{
        let current_time = SystemTime::now();
        let new_list = db::build_list("Test List".to_string(), current_time, current_time);
        //let insert_list = db::create_list(&new_list);
        let _= db::create_list(&new_list);
    }

    let list:Vec<models::List> = models::lists::table
        //filter to where first name equals "jagged"
        .filter(models::lists::title.eq("Test List"))
        //limits output to 5 rows
        .limit(5)
        //put the results into the Person struct
        .select(models::List::as_select())
        //utilize the connection object
        .load(&mut _con)
        .expect("Error");
    println!("{:?}", list);
}
