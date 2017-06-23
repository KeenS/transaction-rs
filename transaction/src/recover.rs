use std::marker::PhantomData;

use {IntoTransaction, Transaction};

pub fn recover<Ctx, A, T, F>(a: A, f: F) -> Recover<A::Tx, T, F>
where
    A: IntoTransaction<Ctx>,
    F: Fn(A::Err) -> A::Item,
{
    Recover {
        tx: a.into_transaction(),
        f: f,
        _phantom: PhantomData,
    }
}

/// The result of `recover`
#[derive(Debug)]
#[must_use]
pub struct Recover<Tx, T, F> {
    tx: Tx,
    f: F,
    _phantom: PhantomData<T>,
}

impl<Tx, F, T> Transaction for Recover<Tx, T, F>
where
    Tx: Transaction,
    F: Fn(Tx::Err) -> Tx::Item,
{
    type Ctx = Tx::Ctx;
    type Item = Tx::Item;
    type Err = Tx::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Recover { ref tx, ref f, .. } = self;
        match tx.run(ctx) {
            r @ Ok(_) => r,
            Err(e) => Ok(f(e)),
        }
    }
}
