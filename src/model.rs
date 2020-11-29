use crate::db;
use crate::error_handler::CustomError;
use crate::schema::sharks;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "sharks"]
pub struct Shark {
    pub id: i32,
    pub name: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Shark {
    pub fn find() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let sharks = sharks::table.load::<Shark>(&conn)?;
        Ok(sharks)
    }

    pub fn get(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let shark = sharks::table.filter(sharks::id.eq(id)).first(&conn)?;
        Ok(shark)
    }

    pub fn create(command: SharkCommand) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let shark = diesel::insert_into(sharks::table)
            .values(command)
            .get_result(&conn)?;
        Ok(shark)
    }

    pub fn update(id: i32, command: SharkCommand) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let shark = diesel::update(sharks::table)
            .filter(sharks::id.eq(id))
            .set(command)
            .get_result(&conn)?;
        Ok(shark)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(sharks::table.filter(sharks::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "sharks"]
pub struct SharkCommand {
    pub name: String,
}
