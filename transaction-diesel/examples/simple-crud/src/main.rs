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
use diesel::result::Error;

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
    // composed computation of DB operations
    // you can get transaction context using `with_ctx`
    let tx = with_ctx(|ctx| -> Result<(), Error> {
        // if you have context, you can run a transaction using `run`.
        // Since it returns a `Result` value, `?` operators can be applied;
        let user = db::create_user("keen").run(ctx)?;
        println!("created user: {:?}", user);
        let res = db::update_user(user.id, "KeenS").run(ctx)?;
        match res {
            None => {
                println!("user not found");
                return Ok(());
            }
            Some(()) => (),
        };
        let updated_user = match db::find_user(user.id).run(ctx)? {
            None => {
                println!("user not found");
                return Ok(());
            }
            Some(u) => u,
        };

        println!("updated user: {:?}", updated_user);
        match db::delete_user(updated_user.id).run(ctx)? {
            None => {
                println!("user not found");
            }
            Some(()) => (),
        };
        Ok(())
    });
    // to run the composed computation, use `transaction_diesel::run`.
    transaction_diesel::run(&conn, tx).unwrap()
}
