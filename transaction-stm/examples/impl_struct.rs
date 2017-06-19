extern crate stm;
extern crate transaction;
extern crate transaction_stm;

use transaction::prelude::*;
use transaction_stm::run;

struct Data {
    x: stm::TVar<i32>,
    y: stm::TVar<i32>,
}

type BoxTx<'a, T> = Box<Transaction<Ctx = stm::Transaction, Item = T, Err = stm::StmError> + 'a>;

impl Data {
    fn inc_x(&self) -> BoxTx<i32> {
        with_ctx(move |ctx: &mut stm::Transaction| {
                     let xv = ctx.read(&self.x)?;
                     ctx.write(&self.x, xv + 1)?;
                     Ok(xv)
                 }).boxed()
    }
    fn inc_y(&self) -> BoxTx<i32> {
        with_ctx(move |ctx: &mut stm::Transaction| {
                     let yv = ctx.read(&self.y)?;
                     ctx.write(&self.y, yv + 1)?;
                     Ok(yv)
                 }).boxed()
    }

    fn inc_xy(&self) -> BoxTx<i32> {
        self.inc_x().and_then(move |_| self.inc_y()).boxed()
    }
    fn add(&self) -> BoxTx<i32> {
        with_ctx(move |ctx: &mut stm::Transaction| {
                     let xv = ctx.read(&self.x)?;
                     let yv = ctx.read(&self.y)?;
                     Ok(xv + yv)
                 }).boxed()
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
