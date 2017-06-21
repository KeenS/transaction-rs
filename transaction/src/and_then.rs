use std::marker::PhantomData;

use {IntoTransaction, Transaction};

pub fn and_then<Ctx, A, F, B>(a: A, f: F) -> AndThen<A::Tx, F, B>
where
    A: IntoTransaction<Ctx>,
    B: IntoTransaction<Ctx, Err = A::Err>,
    F: Fn(A::Item) -> B,
{
    AndThen {
        tx: a.into_transaction(),
        f: f,
        _phantom: PhantomData,
    }
}


/// The result of `and_then`
#[derive(Debug)]
#[must_use]
pub struct AndThen<Tx1, F, Tx2> {
    tx: Tx1,
    f: F,
    _phantom: PhantomData<Tx2>,
}


impl<Tx, Tx2, F> Transaction for AndThen<Tx, F, Tx2>
where
    Tx2: IntoTransaction<Tx::Ctx, Err = Tx::Err>,
    Tx: Transaction,
    F: Fn(Tx::Item) -> Tx2,
{
    type Ctx = Tx::Ctx;
    type Item = Tx2::Item;
    type Err = Tx2::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &AndThen { ref tx, ref f, .. } = self;
        tx.run(ctx).and_then(
            |item| f(item).into_transaction().run(ctx),
        )
    }
}
