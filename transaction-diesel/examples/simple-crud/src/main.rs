#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate transaction;
extern crate transaction_diesel;

mod schema;
mod model;
mod db;

use transaction::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;
    use std::env;
    use diesel::prelude::*;

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let conn = establish_connection();
    let tx = db::create_user("keen").and_then(move |user| {
        println!("created user: {:?}", user);
        db::update_user(user.id, "KeenS")
            .join(ok(user))
            .and_then(|(res, user)| match res {
                None => {
                    println!("user not found");
                    ok(()).branch().left()
                }
                Some(()) => db::find_user(user.id)
                .and_then(move |maybe_updated_user| {
                    match maybe_updated_user {
                        None => {
                            println!("user not found");
                            ok(()).branch().left()
                        },
                        Some(updated_user) => {
                            println!("updated user: {:?}", updated_user);
                            db::delete_user(updated_user.id)
                                .map(|res| match res {
                                    None => {
                                        println!("user not found");
                                    },
                                    Some(()) => ()
                                })
                                .branch().right()
                        }
                    }
                }).branch().right(),

            })
    });

    transaction_diesel::run(&conn, tx).unwrap()
}
