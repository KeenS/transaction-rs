use {IntoTransaction, Transaction};

pub fn join<Ctx, A: IntoTransaction<Ctx>, B: IntoTransaction<Ctx, Err = A::Err>>(
    a: A,
    b: B,
) -> Join<A::Tx, B::Tx> {
    Join {
        tx1: a.into_transaction(),
        tx2: b.into_transaction(),
    }

}


/// The result of `join`
#[derive(Debug)]
#[must_use]
pub struct Join<Tx1, Tx2> {
    tx1: Tx1,
    tx2: Tx2,
}

impl<Tx1, Tx2> Transaction for Join<Tx1, Tx2>
where
    Tx1: Transaction,
    Tx2: Transaction<Ctx = Tx1::Ctx, Err = Tx1::Err>,
{
    type Ctx = Tx1::Ctx;
    type Item = (Tx1::Item, Tx2::Item);
    type Err = Tx1::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Join { ref tx1, ref tx2, .. } = self;
        match (tx1.run(ctx), tx2.run(ctx)) {
            (Ok(r1), Ok(r2)) => Ok((r1, r2)),
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}
