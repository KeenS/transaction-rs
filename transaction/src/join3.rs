use {IntoTransaction, Transaction};

pub fn join3<
    Ctx,
    A: IntoTransaction<Ctx>,
    B: IntoTransaction<Ctx, Err = A::Err>,
    C: IntoTransaction<Ctx, Err = A::Err>,
>(
    a: A,
    b: B,
    c: C,
) -> Join3<A::Tx, B::Tx, C::Tx> {
    Join3 {
        tx1: a.into_transaction(),
        tx2: b.into_transaction(),
        tx3: c.into_transaction(),
    }

}

/// The result of `join3`
#[derive(Debug)]
#[must_use]
pub struct Join3<Tx1, Tx2, Tx3> {
    tx1: Tx1,
    tx2: Tx2,
    tx3: Tx3,
}

impl<Tx1, Tx2, Tx3> Transaction for Join3<Tx1, Tx2, Tx3>
where
    Tx1: Transaction,
    Tx2: Transaction<
        Ctx = Tx1::Ctx,
        Err = Tx1::Err,
    >,
    Tx3: Transaction<
        Ctx = Tx1::Ctx,
        Err = Tx1::Err,
    >,
{
    type Ctx = Tx1::Ctx;
    type Item = (Tx1::Item, Tx2::Item, Tx3::Item);
    type Err = Tx1::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Join3 {
            ref tx1,
            ref tx2,
            ref tx3,
        } = self;
        match (tx1.run(ctx), tx2.run(ctx), tx3.run(ctx)) {
            (Ok(r1), Ok(r2), Ok(r3)) => Ok((r1, r2, r3)),
            (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => Err(e),
        }
    }
}
