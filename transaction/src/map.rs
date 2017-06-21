use {IntoTransaction, Transaction};

pub fn map<Ctx, A, F, B>(a: A, f: F) -> Map<A::Tx, F>
where
    A: IntoTransaction<Ctx>,
    F: Fn(A::Item) -> B,
{
    Map {
        tx: a.into_transaction(),
        f: f,
    }
}



/// The result of `map`
#[derive(Debug)]
#[must_use]
pub struct Map<Tx, F> {
    tx: Tx,
    f: F,
}
impl<Tx, U, F> Transaction for Map<Tx, F>
where
    Tx: Transaction,
    F: Fn(Tx::Item) -> U,
{
    type Ctx = Tx::Ctx;
    type Item = U;
    type Err = Tx::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Map { ref tx, ref f } = self;
        tx.run(ctx).map(f)
    }
}
