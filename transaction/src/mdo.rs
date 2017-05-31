use super::prelude::*;

/// bind for Transaction<Ctx>, equivalent to `tx.and_then(f)
pub fn bind<Ctx, Tx, F, B>(tx: Tx, f: F) -> ::AndThen<Tx, F, B>
    where B: Transaction<Ctx, Err = Tx::Err>,
          F: Fn(Tx::Item) -> B,
          Tx: Transaction<Ctx> + Sized
{
    tx.and_then(f)
}

/// return for Transaction<Ctx, Item = T, Err = E>, equivalent to `ok(x)`
pub fn ret<T, E>(x: T) -> ::TxOk<T, E> {
    ok(x)
}
