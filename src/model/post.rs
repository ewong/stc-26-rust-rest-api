use diesel::prelude::*;
use serde::Serialize;

use crate::db::schema::posts;
use crate::db::schema::posts::columns::body as col_body;
use crate::db::schema::posts::columns::deleted as col_deleted;
use crate::db::schema::posts::columns::published as col_published;
use crate::db::schema::posts::columns::slug as col_slug;
use crate::db::schema::posts::columns::title as col_title;
use crate::db::schema::posts::dsl::posts as dsl_posts;

#[derive(Serialize, Clone, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub published: bool,
    pub deleted: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
struct NewPost<'a> {
    pub title: &'a str,
    pub slug: &'a str,
    pub body: &'a str,
}

fn slugify(title: &str) -> String {
    let t = String::from(title);
    t.to_lowercase().replace(" ", "-")
}

impl Post {
    pub fn create(title: &str, body: &str, dbconn: &PgConnection) -> Post {
        diesel::insert_into(posts::table)
            .values(NewPost {
                title,
                slug: &slugify(title),
                body,
            })
            .get_result::<Post>(dbconn)
            .expect("Error saving new post")
    }

    pub fn find(dbconn: &PgConnection) -> Vec<Post> {
        dsl_posts
            .filter(col_deleted.eq(false))
            .limit(50)
            .load::<Post>(dbconn)
            .expect("Error loading posts")
    }

    pub fn get(slug: &str, dbconn: &PgConnection) -> Post {
        dsl_posts
            .filter(col_slug.eq(slug))
            .filter(col_deleted.eq(false))
            .get_result::<Post>(dbconn)
            .expect("Error finding post")
    }

    pub fn update(
        slug: &str,
        title: &str,
        body: &str,
        published: bool,
        dbconn: &PgConnection,
    ) -> Post {
        diesel::update(
            posts::table
                .filter(col_slug.eq(slug))
                .filter(col_deleted.eq(false)),
        )
        .set((
            col_title.eq(title),
            col_slug.eq(&slugify(title)),
            col_body.eq(body),
            col_published.eq(published),
        ))
        .get_result::<Post>(dbconn)
        .expect("Error updating post")
    }

    pub fn delete(slug: &str, dbconn: &PgConnection) -> bool {
        let num_rows = diesel::update(
            posts::table
                .filter(col_slug.eq(slug))
                .filter(col_deleted.eq(false)),
        )
        .set(col_deleted.eq(true))
        .execute(dbconn)
        .expect("Error deleting post");
        num_rows > 0
    }
}
