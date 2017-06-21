use std::marker::PhantomData;

use {IntoTransaction, Transaction};

pub fn try_recover<Ctx, A, F, B>(a: A, f: F) -> TryRecover<A::Tx, F, B>
where
    A: IntoTransaction<Ctx>,
    F: Fn(A::Item) -> Result<B, A::Err>,
{
    TryRecover {
        tx: a.into_transaction(),
        f: f,
        _phantom: PhantomData,
    }

}

/// The result of `try_recover`
#[derive(Debug)]
#[must_use]
pub struct TryRecover<Tx, F, B> {
    tx: Tx,
    f: F,
    _phantom: PhantomData<B>,
}

impl<Tx, F, B> Transaction for TryRecover<Tx, F, B>
where
    Tx: Transaction,
    F: Fn(Tx::Err) -> Result<Tx::Item, B>,
{
    type Ctx = Tx::Ctx;
    type Item = Tx::Item;
    type Err = B;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let TryRecover { ref tx, ref f, .. } = *self;
        match tx.run(ctx) {
            Ok(r) => Ok(r),
            Err(e) => f(e),
        }
    }
}
