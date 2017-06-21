use {IntoTransaction, Transaction};

pub fn join4<
    Ctx,
    A: IntoTransaction<Ctx>,
    B: IntoTransaction<Ctx, Err = A::Err>,
    C: IntoTransaction<Ctx, Err = A::Err>,
    D: IntoTransaction<Ctx, Err = A::Err>,
>(
    a: A,
    b: B,
    c: C,
    d: D,
) -> Join4<A::Tx, B::Tx, C::Tx, D::Tx> {
    Join4 {
        tx1: a.into_transaction(),
        tx2: b.into_transaction(),
        tx3: c.into_transaction(),
        tx4: d.into_transaction(),
    }

}

/// The result of `join4`
#[derive(Debug)]
#[must_use]
pub struct Join4<Tx1, Tx2, Tx3, Tx4> {
    tx1: Tx1,
    tx2: Tx2,
    tx3: Tx3,
    tx4: Tx4,
}

impl<Tx1, Tx2, Tx3, Tx4> Transaction for Join4<Tx1, Tx2, Tx3, Tx4>
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
    Tx4: Transaction<
        Ctx = Tx1::Ctx,
        Err = Tx1::Err,
    >,
{
    type Ctx = Tx1::Ctx;
    type Item = (Tx1::Item, Tx2::Item, Tx3::Item, Tx4::Item);
    type Err = Tx1::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Join4 {
            ref tx1,
            ref tx2,
            ref tx3,
            ref tx4,
        } = self;
        match (tx1.run(ctx), tx2.run(ctx), tx3.run(ctx), tx4.run(ctx)) {
            (Ok(r1), Ok(r2), Ok(r3), Ok(r4)) => Ok((r1, r2, r3, r4)),
            (Err(e), _, _, _) |
            (_, Err(e), _, _) |
            (_, _, Err(e), _) |
            (_, _, _, Err(e)) => Err(e),
        }
    }
}
