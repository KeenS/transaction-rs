use {IntoTransaction, Transaction};

pub fn map_err<Ctx, A, F, B>(a: A, f: F) -> MapErr<A::Tx, F>
where
    A: IntoTransaction<Ctx>,
    F: Fn(A::Err) -> B,
{
    MapErr {
        tx: a.into_transaction(),
        f: f,
    }
}


/// The result of `map_err`
#[derive(Debug)]
#[must_use]
pub struct MapErr<Tx, F> {
    tx: Tx,
    f: F,
}

impl<E, Tx, F> Transaction for MapErr<Tx, F>
where
    Tx: Transaction,
    F: Fn(Tx::Err) -> E,
{
    type Ctx = Tx::Ctx;
    type Item = Tx::Item;
    type Err = E;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &MapErr { ref tx, ref f } = self;
        tx.run(ctx).map_err(f)
    }
}
