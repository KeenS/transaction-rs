use super::prelude::*;

/// bind for Transaction>, equivalent to `tx.and_then(f)
pub fn bind<Tx, F, B>(tx: Tx, f: F) -> ::AndThen<Tx, F, B>
    where B: Transaction<Ctx = Tx::Ctx, Err = Tx::Err>,
          F: Fn(Tx::Item) -> B,
          Tx: Transaction + Sized
{
    tx.and_then(f)
}

/// return for Transaction<Ctx = Ctx, Item = T, Err = E>, equivalent to `ok(x)`
pub fn ret<Ctx, T, E>(x: T) -> ::TxOk<Ctx, T, E> {
    ok(x)
}
