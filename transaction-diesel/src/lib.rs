//! A transaction runner for diesel

extern crate diesel;
extern crate transaction;
use transaction::*;
use std::marker::PhantomData;

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

    fn conn(&self) -> &Cn {
        &self.conn
    }
}

/// Receive the connection from the executing transaction and perform computation.
pub fn with_conn<Conn, F, T, E>(f: F) -> WithConn<Conn, F>
    where F: Fn(&Conn) -> Result<T, E>
{
    WithConn {
        f: f,
        _phantom: PhantomData,
    }
}

/// The result of `with_conn`
#[derive(Debug)]
pub struct WithConn<Conn, F> {
    f: F,
    _phantom: PhantomData<Conn>,
}

impl<'a, Conn, T, E, F> Transaction<DieselContext<'a, Conn>> for WithConn<Conn, F>
    where F: Fn(&Conn) -> Result<T, E>
{
    type Item = T;
    type Err = E;
    fn run(&self, ctx: &mut DieselContext<'a, Conn>) -> Result<Self::Item, Self::Err> {
        (self.f)(ctx.conn())
    }
}



/// run a transaction within the given connection.
pub fn run<'a, Cn, T, E, Tx>(cn: &'a Cn, tx: Tx) -> Result<T, E>
    where Cn: diesel::Connection,
          E: From<diesel::result::Error>,
          Tx: Transaction<DieselContext<'a, Cn>, Item = T, Err = E>
{
    cn.clone()
        .transaction(|| tx.run(&mut DieselContext::new(cn)))
}
