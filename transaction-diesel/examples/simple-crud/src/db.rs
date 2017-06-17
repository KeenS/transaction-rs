use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use transaction::prelude::*;
use transaction_diesel::DieselContext;
use transaction_diesel::with_conn;

use model::*;

type Ctx<'a> = DieselContext<'a, PgConnection>;
// Until Rust supports `impl Trait`, we need to box `Transaction`s when returning from functions.
type BoxTx<'a, T> = Box<Transaction<Ctx = Ctx<'a>, Item = T, Err = Error> + 'a>;

pub fn create_user<'a>(name: &'a str) -> BoxTx<'a, User> {
    use schema::users::table;
    // Connections are injected via transaction.
    // Get it using `with_conn`
    with_conn(move |cn| {
                  diesel::insert(&NewUser { name: name })
                      .into(table)
                      .get_result(cn)
    })
        // box it
        .boxed()
}

pub fn find_user<'a>(id: i64) -> BoxTx<'a, Option<User>> {
    use schema::users::dsl::users;
    with_conn(move |cn| users.find(id).get_result(cn).optional()).boxed()
}

pub fn update_user<'a>(id: i64, name: &'a str) -> BoxTx<'a, Option<()>> {
    use schema::users::dsl;
    with_conn(move |cn| {
                  diesel::update(dsl::users.find(id))
                      .set(dsl::name.eq(name))
                      .execute(cn)
                      .map(|_| ())
                      .optional()
              }).boxed()
}


pub fn delete_user<'a>(id: i64) -> BoxTx<'a, Option<()>> {
    use schema::users::dsl::users;
    with_conn(move |cn| {
                  diesel::delete(users.find(id))
                      .execute(cn)
                      .map(|_| ())
                      .optional()
              }).boxed()
}
