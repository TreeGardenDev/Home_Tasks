use diesel::prelude::*;
use std::time::SystemTime;

/*
  Sql Statements
    CREATE TABLE lists (
  id SERIAL PRIMARY KEY,
  title VARCHAR,
  completed BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ,
  due_date TIMESTAMP,
  number_of_items INTEGER DEFAULT 0,y
  owner VARCHAR(255),
  parent_item_id INTEGER REFERENCES items(id) ON DELETE CASCADE
);

CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed BOOLEAN DEFAULT FALSE,
    required BOOLEAN DEFAULT FALSE,
    list_id INTEGER REFERENCES lists(id) ON DELETE CASCADE,
    child_list_id INTEGER REFERENCES lists(id) ON DELETE CASCADE
);
*/


diesel::table! {
    items (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        completed -> Nullable<Bool>,
        required -> Nullable<Bool>,
        list_id -> Nullable<Int4>,
        child_list_id -> Nullable<Int4>,
    }
}

diesel::table! {
    lists (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        completed -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        due_date -> Nullable<Timestamp>,
        number_of_items -> Nullable<Int4>,
        #[max_length = 255]
        owner -> Nullable<Varchar>,
        parent_item_id -> Nullable<Int4>,
    }
}

diesel::joinable!(items -> lists (list_id));

diesel::allow_tables_to_appear_in_same_query!(
    items,
    lists,
);


/*
        id -> Int4,
        title -> Varchar,
        completed -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        due_date -> Nullable<Timestamp>,
        number_of_items -> Nullable<Int4>,
        #[max_length = 255]
        owner -> Varchar
*/
#[derive(Queryable, Selectable,Debug)]
#[diesel(table_name = lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct List{
    pub id: i32,
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub due_date: Option<SystemTime>,
    pub number_of_items: Option<i32>,
    pub owner: Option<String>,
    pub parent_item_id: Option<i32>
}

#[derive(Insertable, Debug)]
#[diesel(table_name=lists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewList{
    pub title: Option<String>,
    pub completed: bool,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub due_date: Option<SystemTime>,
    pub number_of_items: i32,
    pub owner: Option<String>,
    pub parent_item_id: Option<i32>
}
impl NewList {
    pub fn new() -> NewList {
        NewList {
            title: None,
            completed: false,
            created_at: None,
            updated_at: None,
            due_date: None,
            number_of_items: 0,
            parent_item_id: None,
            owner: None
        }
    }
}

/*
        id -> Int4,
        title -> Varchar,
        completed -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        due_date -> Nullable<Timestamp>,
        number_of_items -> Nullable<Int4>,
        #[max_length = 255]
        owner -> Varchar,
*/

#[derive(Queryable, Selectable,Debug,AsChangeset)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item{
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub completed: Option<bool>,
    pub required: Option<bool>,
    pub list_id: Option<i32>,
    pub child_list_id: Option<i32>,
}

#[derive(Insertable,Selectable,Debug,AsChangeset)]
#[diesel(table_name = items)]
pub struct NewItem{
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub completed: Option<bool>,
    pub required: Option<bool>,
    pub list_id: Option<i32>,
    pub child_list_id: Option<i32>,
}
impl NewItem {
    pub fn new() -> NewItem {
        NewItem {
            name: String::new(),
            description: None,
            created_at: None,
            updated_at: None,
            completed: Some(false),
            required: Some(false),
            list_id: None,
            child_list_id: None,
        }
    }
}



