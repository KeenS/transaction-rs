use std::marker::PhantomData;

use {IntoTransaction, Transaction};



pub fn retry<Ctx, F, Tx>(n: usize, f: F) -> Retry<Ctx, F, Tx>
where
    Tx: IntoTransaction<Ctx>,
    F: Fn(usize) -> Tx,
{
    Retry {
        n: n,
        f: f,
        _phantom: PhantomData,
    }
}

/// The result of `retry`
#[derive(Debug)]
#[must_use]
pub struct Retry<Ctx, F, Tx> {
    n: usize,
    f: F,
    _phantom: PhantomData<(Tx, Ctx)>,
}

impl<Ctx, F, Tx> Transaction for Retry<Ctx, F, Tx>
where
    F: Fn(usize) -> Tx,
    Tx: IntoTransaction<Ctx>,
{
    type Ctx = Ctx;
    type Item = Tx::Item;
    type Err = Vec<Tx::Err>;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let Retry { ref n, ref f, .. } = *self;
        let mut ret = Vec::new();
        for i in 0..*n {
            let t = match f(i).into_transaction().run(ctx) {
                Ok(t) => return Ok(t),
                Err(e) => e,
            };
            ret.push(t);
        }
        Err(ret)
    }
}
