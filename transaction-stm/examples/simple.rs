extern crate stm;
extern crate transaction;
extern crate transaction_stm;

use transaction::prelude::*;
use transaction_stm::{run, with_tx};

fn main() {
    let x = stm::TVar::new(0);
    let y = stm::TVar::new(0);

    let inc_xy = with_tx(|ctx| {
                             let xv = ctx.read(&x)?;
                             ctx.write(&x, xv + 1)?;
                             Ok(xv)
                         }).and_then(|_| {
                      with_tx(|ctx| {
                                  let yv = ctx.read(&y)?;
                                  ctx.write(&y, yv + 1)?;
                                  Ok(yv)
                              })
                  })
        .and_then(|_| with_tx(|ctx| Ok(ctx.read(&x)? + ctx.read(&y)?)));
    let ret = run(&inc_xy);
    assert_eq!(ret, 2);

}
