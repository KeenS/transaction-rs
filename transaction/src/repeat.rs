use std::marker::PhantomData;

use {IntoTransaction, Transaction};

pub fn repeat<Ctx, F, Tx>(n: usize, f: F) -> Repeat<Ctx, F, Tx>
where
    Tx: IntoTransaction<Ctx>,
    F: Fn(usize) -> Tx,
{
    Repeat {
        n: n,
        f: f,
        _phantom: PhantomData,
    }
}

/// The result of `repeat`
#[derive(Debug)]
#[must_use]
pub struct Repeat<Ctx, F, Tx> {
    n: usize,
    f: F,
    _phantom: PhantomData<(Tx, Ctx)>,
}

impl<Ctx, F, Tx> Transaction for Repeat<Ctx, F, Tx>
where
    F: Fn(usize) -> Tx,
    Tx: IntoTransaction<Ctx>,
{
    type Ctx = Ctx;
    type Item = Vec<Tx::Item>;
    type Err = Tx::Err;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let Repeat { ref n, ref f, .. } = *self;
        let mut ret = Vec::new();
        for i in 0..*n {
            let t = f(i).into_transaction().run(ctx)?;
            ret.push(t);
        }
        Ok(ret)
    }
}
