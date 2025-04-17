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
  owner VARCHAR(255)
);

CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed BOOLEAN DEFAULT FALSE,
    required BOOLEAN DEFAULT FALSE,
    list_id INTEGER REFERENCES lists(id) ON DELETE CASCADE
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
    pub owner: Option<String>
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

#[derive(Queryable, Selectable,Debug)]
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
}
