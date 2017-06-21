use std::marker::PhantomData;

use {IntoTransaction, Transaction};

pub fn then<Ctx, A, F, B, Tx2>(a: A, f: F) -> Then<A::Tx, F, Tx2>
where
    A: IntoTransaction<Ctx>,
    Tx2: IntoTransaction<Ctx, Item = B, Err = A::Err>,
    F: Fn(Result<A::Item, A::Err>) -> Tx2,
{
    Then {
        tx: a.into_transaction(),
        f: f,
        _phantom: PhantomData,
    }
}

/// The result of `then`
#[derive(Debug)]
#[must_use]
pub struct Then<Tx1, F, Tx2> {
    tx: Tx1,
    f: F,
    _phantom: PhantomData<Tx2>,
}

impl<Tx, Tx2, F> Transaction for Then<Tx, F, Tx2>
where
    Tx2: IntoTransaction<Tx::Ctx, Err = Tx::Err>,
    Tx: Transaction,
    F: Fn(Result<Tx::Item, Tx::Err>) -> Tx2,
{
    type Ctx = Tx::Ctx;
    type Item = Tx2::Item;
    type Err = Tx2::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Then { ref tx, ref f, .. } = self;
        f(tx.run(ctx)).into_transaction().run(ctx)
    }
}
