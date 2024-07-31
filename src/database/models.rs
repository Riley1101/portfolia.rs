use core::fmt;
use std::fmt::{Display, Formatter};

use super::schema;
use chrono::NaiveDateTime;
use diesel::{prelude::*, r2d2};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::articles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub thumbnail_image: Option<String>,
    pub createdat: NaiveDateTime,
    pub updatedat: NaiveDateTime,
}

impl Article {
    fn print(&self) {
        println!("Article: {:#?}", self.title);
    }
}
