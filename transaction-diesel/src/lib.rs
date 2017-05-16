//! A transaction runner for diesel

extern crate diesel;
extern crate transaction;
use transaction::Transaction;
use std::marker::PhantomData;
use std::ops::Deref;

/// diesel transaction object.
pub struct DieselContext<'a, Cn: 'a> {
    pub conn: &'a Cn,
    _phantom: PhantomData<()>,
}

impl<'a, Cn: 'a> DieselContext<'a, Cn> {
    // never pub this function
    fn new(conn: &'a Cn) -> Self {
        DieselContext {
            conn: conn,
            _phantom: PhantomData,
        }
    }
}

impl<'a, Cn: 'a> Deref for DieselContext<'a, Cn> {
    type Target = Cn;
    fn deref(&self) -> &Self::Target {
        self.conn
    }
}

/// run a transaction within the given connection.
pub fn run<'a, Cn, T, E, Tx>(cn: &'a Cn, tx: Tx) -> Result<T, E>
    where Cn: diesel::Connection,
          E: From<diesel::result::Error>,
          Tx: Transaction<DieselContext<'a, Cn>, Item = T, Err = E>
{
    let mut cn2 = DieselContext::new(cn.clone());
    cn.transaction(|| tx.run(&mut cn2))
}
