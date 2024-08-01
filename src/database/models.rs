#![allow(dead_code)]
use super::schema;
use crate::articles;
use chrono::NaiveDateTime;
use core::fmt;
use diesel::dsl::{AsSelect, Select};
use diesel::pg::Pg;
use diesel::{prelude::*, r2d2};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::ser::{Serialize, SerializeStruct};
use std::fmt::{Display, Formatter};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub type PoolConnection = r2d2::PooledConnection<r2d2::ConnectionManager<PgConnection>>;

pub type All = Select<articles, AsSelect<Article, Pg>>;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::articles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: Option<String>,
    pub body: String,
    pub published: bool,
    pub thumbnail_image: Option<String>,
    pub createdat: NaiveDateTime,
    pub updatedat: NaiveDateTime,
}

impl Serialize for Article {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Article", 7)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("body", &self.body)?;
        state.serialize_field("published", &self.published)?;
        state.serialize_field("thumbnail_image", &self.thumbnail_image)?;
        state.serialize_field("slug", &self.slug)?;
        let createdat = self.createdat.format("%Y-%m-%d").to_string();
        state.serialize_field("createdat", &createdat)?;
        let updatedat = self.updatedat.format("%Y-%m-%d").to_string();
        state.serialize_field("updatedat", &updatedat)?;
        state.end()
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "--- \n id: {} \n title: {} \n published: {} \n updatedat: {} \n",
            self.title, self.title, self.published, self.updatedat
        )
    }
}

pub trait ArticleCRUD {
    fn get_all_articles(conn: PoolConnection) -> Vec<Article>;
    fn get_all_article_preview(conn: PoolConnection) -> Article;
    fn get_article_by_slug(conn: PoolConnection, slug: String) -> Article;
}

impl ArticleCRUD for Article {
    // add code here
    fn get_all_articles(mut conn: PoolConnection) -> Vec<Article> {
        use schema::articles::dsl::*;
        articles
            .filter(published.eq(true))
            .limit(5)
            .select(Article::as_select())
            .load(&mut conn)
            .expect("error loading posts")
    }

    fn get_all_article_preview(mut conn: PoolConnection) -> Article {
        use schema::articles::dsl::*;
        articles
            .select(Article::as_select())
            .first(&mut conn)
            .expect("error loading posts")
    }

    fn get_article_by_slug(mut conn: PoolConnection, article_slug: String) -> Article {
        use schema::articles::dsl::*;
        articles
            .filter(slug.eq(article_slug))
            .select(Article::as_select())
            .first(&mut conn)
            .expect("error loading posts")
    }
}
