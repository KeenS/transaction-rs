use std::marker::PhantomData;

use {IntoTransaction, Transaction};

pub fn try_abort<Ctx, A, F, B>(a: A, f: F) -> TryAbort<A::Tx, F, B>
where
    A: IntoTransaction<Ctx>,
    F: Fn(A::Item) -> Result<B, A::Err>,
{
    TryAbort {
        tx: a.into_transaction(),
        f: f,
        _phantom: PhantomData,
    }
}


#[derive(Debug)]
#[must_use]
pub struct TryAbort<Tx, F, B> {
    tx: Tx,
    f: F,
    _phantom: PhantomData<B>,
}

impl<Tx, F, B> Transaction for TryAbort<Tx, F, B>
where
    Tx: Transaction,
    F: Fn(Tx::Item) -> Result<B, Tx::Err>,
{
    type Ctx = Tx::Ctx;
    type Item = B;
    type Err = Tx::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let TryAbort { ref tx, ref f, .. } = *self;
        match tx.run(ctx) {
            Ok(r) => f(r),
            Err(e) => Err(e),
        }
    }
}
