//! Run the `stm` transaction
//!
//! # Examples
//! ```rust
//! extern crate stm;
//! extern crate transaction;
//! extern crate transaction_stm;
//!
//! use transaction::{Transaction, with_ctx};
//! use transaction_stm::run;
//!
//! fn main() {
//!     let x = stm::TVar::new(0);
//!     let y = stm::TVar::new(0);
//!
//!     let inc_xy =
//!         with_ctx(|ctx: &mut stm::Transaction| {
//!                      let xv = ctx.read(&x)?;
//!                      ctx.write(&x, xv + 1)?;
//!                      Ok(xv)
//!                  })
//!                 .and_then(|_| {
//!                               with_ctx(|ctx: &mut stm::Transaction| {
//!                                            let yv = ctx.read(&y)?;
//!                                            ctx.write(&y, yv + 1)?;
//!                                            Ok(yv)
//!                                        })
//!                           })
//!                 .and_then(|_| {
//!                               with_ctx(|ctx: &mut stm::Transaction| {
//!                                            Ok(ctx.read(&x)? + ctx.read(&y)?)
//!                                        })
//!                           });
//!     let ret = run(&inc_xy);
//!     assert_eq!(ret, 2);
//!
//! }
//! ```



extern crate stm;
extern crate transaction;

use transaction::Transaction;
use stm::Transaction as Stm;


/// Run the `stm` transaction
pub fn run<T, Tx>(tx: &Tx) -> T
    where Tx: Transaction<Ctx = Stm, Item = T, Err = stm::StmError>
{
    Stm::with(|stm| tx.run(stm))
}

pub fn with_tx<F, T, E>(f: F) -> WithTx<F>
    where F: Fn(&mut Stm) -> Result<T, E>
{
    WithTx { f: f }
}

pub struct WithTx<F> {
    f: F,
}

impl<F, T, E> Transaction for WithTx<F>
    where F: Fn(&mut Stm) -> Result<T, E>
{
    type Ctx = Stm;
    type Item = T;
    type Err = E;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let WithTx { ref f } = *self;
        f(ctx)
    }
}
