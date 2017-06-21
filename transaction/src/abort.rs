use std::marker::PhantomData;

use {IntoTransaction, Transaction};

/// Take the previous successfull value of computation and abort the
/// transaction.
pub fn abort<Ctx, A, T, F>(a: A, f: F) -> Abort<A::Tx, T, F>
where
    A: IntoTransaction<Ctx>,
    F: Fn(A::Item) -> A::Err,
{
    Abort {
        tx: a.into_transaction(),
        f: f,
        _phantom: PhantomData,
    }
}


/// The result of `abort`
#[derive(Debug)]
#[must_use]
pub struct Abort<Tx, T, F> {
    tx: Tx,
    f: F,
    _phantom: PhantomData<T>,
}

impl<Tx, F, T> Transaction for Abort<Tx, T, F>
where
    Tx: Transaction,
    F: Fn(Tx::Item) -> Tx::Err,
{
    type Ctx = Tx::Ctx;
    type Item = T;
    type Err = Tx::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Abort { ref tx, ref f, .. } = self;
        match tx.run(ctx) {
            Ok(r) => Err(f(r)),
            Err(e) => Err(e),
        }
    }
}
