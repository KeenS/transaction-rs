use std::marker::PhantomData;

use {IntoTransaction, Transaction};


pub fn or_else<Ctx, A, F, B>(a: A, f: F) -> OrElse<A::Tx, F, B>
where
    A: IntoTransaction<Ctx>,
    B: IntoTransaction<Ctx, Item = A::Item>,
    F: Fn(A::Err) -> B,
{
    OrElse {
        tx: a.into_transaction(),
        f: f,
        _phantom: PhantomData,
    }
}


/// The result of `or_else`
#[derive(Debug)]
#[must_use]
pub struct OrElse<Tx1, F, Tx2> {
    tx: Tx1,
    f: F,
    _phantom: PhantomData<Tx2>,
}

impl<Tx, Tx2, F> Transaction for OrElse<Tx, F, Tx2>
where
    Tx2: IntoTransaction<
        Tx::Ctx,
        Item = Tx::Item,
        Err = Tx::Err,
    >,
    Tx: Transaction,
    F: Fn(Tx::Err) -> Tx2,
{
    type Ctx = Tx::Ctx;
    type Item = Tx::Item;
    type Err = Tx::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &OrElse { ref tx, ref f, .. } = self;
        tx.run(ctx).or_else(
            |item| f(item).into_transaction().run(ctx),
        )
    }
}
