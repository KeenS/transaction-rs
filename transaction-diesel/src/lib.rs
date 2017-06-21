//! A transaction runner for diesel

extern crate diesel;
extern crate transaction;
use transaction::*;
use std::marker::PhantomData;

/// run the given function insed a transaction using the given connection.
pub fn run<'a, Cn, T, E, Tx>(cn: &'a Cn, tx: Tx) -> Result<T, E>
where
    Cn: diesel::Connection,
    E: From<diesel::result::Error>,
    Tx: Transaction<Ctx = DieselContext<'a, Cn>, Item = T, Err = E>,
{
    cn.clone().transaction(
        || tx.run(&mut DieselContext::new(cn)),
    )
}

/// run the given function insed a transaction using the given connection but do not commit it.
/// Panics if the given function returns an Err.
/// This is usefull for testing
pub fn test_run<'a, Cn, T, E, Tx>(cn: &'a Cn, tx: Tx) -> T
where
    Cn: diesel::Connection,
    E: From<diesel::result::Error>,
    Tx: Transaction<Ctx = DieselContext<'a, Cn>, Item = T, Err = E>,
{
    cn.clone().test_transaction(
        || tx.run(&mut DieselContext::new(cn)),
    )
}

/// diesel transaction object.
pub struct DieselContext<'a, Cn: 'a> {
    conn: &'a Cn,
    _phantom: PhantomData<()>,
}

impl<'a, Cn> DieselContext<'a, Cn> {
    // never pub this function
    fn new(conn: &'a Cn) -> Self {
        DieselContext {
            conn: conn,
            _phantom: PhantomData,
        }
    }

    fn conn(&self) -> &'a Cn {
        &self.conn
    }
}

/// Receive the connection from the executing transaction and perform computation.
pub fn with_conn<'a, Conn, F, T, E>(f: F) -> WithConn<'a, Conn, F>
where
    F: Fn(&'a Conn) -> Result<T, E>,
{
    WithConn {
        f: f,
        _phantom: PhantomData,
    }
}

/// The result of `with_conn`
#[derive(Debug)]
pub struct WithConn<'a, Conn: 'a, F> {
    f: F,
    _phantom: PhantomData<&'a Conn>,
}

impl<'a, Conn, T, E, F> Transaction for WithConn<'a, Conn, F>
where
    F: Fn(&'a Conn) -> Result<T, E>,
{
    type Ctx = DieselContext<'a, Conn>;
    type Item = T;
    type Err = E;
    fn run(&self, ctx: &mut DieselContext<'a, Conn>) -> Result<Self::Item, Self::Err> {
        (self.f)(ctx.conn())
    }
}
