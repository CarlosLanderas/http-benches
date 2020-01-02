#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod models;
mod schema;

use actix_web::{HttpServer, web, App, post, Result, Error, error, HttpResponse};
use diesel::{PgConnection, RunQueryDsl, ExpressionMethods};
use diesel::r2d2::ConnectionManager;
use crate::models::{User, NewUser};
use uuid::Uuid;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[rustfmt::skip]
#[actix_rt::main]
async fn main() -> std::io::Result<()>  {

    let manager = ConnectionManager::<PgConnection>
                ::new("postgres://admin:example@localhost");
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");


    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(post_user)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[post("/user")]
async fn post_user(pool: web::Data<Pool>) -> Result<String> {

    let insert = web::block(move || {
        add_user(pool)
    }).await
      .map_err(|e| error::ErrorBadRequest(e))?;

    Ok(insert.to_string())
}

fn add_user(pool: web::Data<Pool>) -> Result<i32, diesel::result::Error> {

    use self::schema::users::dsl::*;

    let user =  NewUser {
        email: format!("{}{}", Uuid::new_v4().to_string(), "@host.com"),
        is_enabled: true
    };

    let conn = pool.get().unwrap();

    let new_user : User = diesel::
        insert_into(schema::users::table)
        .values(user)
        .get_result(&conn)?;

    Ok(new_user.id)
}
