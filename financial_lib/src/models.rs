use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::records)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Record {
    pub id: Option<i32>,
    pub money_type: String, 
    pub amount: f32,
    pub expense: Option<String>, 
    pub time: String,   
}

use crate::schema::records;
#[derive(Insertable)]
#[diesel(table_name = records)]
pub struct NewRecord<'a> {
    pub money_type: &'a str,
    pub amount: f32,
    pub expense: Option<&'a str>,
    pub time: &'a str,
}