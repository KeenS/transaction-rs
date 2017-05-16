extern crate diesel;
extern crate transaction;
use transaction::Transaction;
use std::marker::PhantomData;
use std::ops::Deref;

pub struct DieselContext<'a, Cn: 'a> {
    conn: &'a Cn,
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

pub fn run<'a, Cn, T, E, Tx>(cn: &'a Cn, tx: Tx) -> Result<T, E>
    where Cn: diesel::Connection,
          E: From<diesel::result::Error>,
          Tx: Transaction<DieselContext<'a, Cn>, Item = T, Err = E>
{
    let mut cn2 = DieselContext::new(cn.clone());
    cn.transaction(|| tx.run(&mut cn2))
}
