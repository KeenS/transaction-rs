#![feature(conservative_impl_trait)]


extern crate stm;
extern crate transaction;
extern crate transaction_stm;

use transaction::{Transaction, with_ctx};
use transaction_stm::run;

struct Data {
    x: stm::TVar<i32>,
    y: stm::TVar<i32>,
}

impl Data {
    fn inc_x<'a>(&'a self)
                 -> impl 'a + Transaction<stm::Transaction, Item = i32, Err = stm::StmError> {
        with_ctx(move |ctx: &mut stm::Transaction| {
                     let xv = ctx.read(&self.x)?;
                     ctx.write(&self.x, xv + 1)?;
                     Ok(xv)
                 })
    }
    fn inc_y<'a>(&'a self)
                 -> impl 'a + Transaction<stm::Transaction, Item = i32, Err = stm::StmError> {
        with_ctx(move |ctx: &mut stm::Transaction| {
                     let yv = ctx.read(&self.y)?;
                     ctx.write(&self.y, yv + 1)?;
                     Ok(yv)
                 })
    }

    fn inc_xy<'a>(&'a self)
                  -> impl 'a + Transaction<stm::Transaction, Item = i32, Err = stm::StmError> {
        self.inc_x().and_then(move |_| self.inc_y())
    }
    fn add<'a>(&'a self)
               -> impl 'a + Transaction<stm::Transaction, Item = i32, Err = stm::StmError> {
        with_ctx(move |ctx: &mut stm::Transaction| {
                     let xv = ctx.read(&self.x)?;
                     let yv = ctx.read(&self.y)?;
                     Ok(xv + yv)
                 })
    }
}




fn main() {
    let data = Data {
        x: stm::TVar::new(0),
        y: stm::TVar::new(0),
    };

    let ret = run(data.inc_xy().and_then(|_| data.add()));

    assert_eq!(ret, 2);
}
