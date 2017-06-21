use std::marker::PhantomData;

use Transaction;

/// make a error transaction value.
pub fn err<Ctx, T, E>(e: E) -> TxErr<Ctx, T, E> {
    TxErr {
        err: e,
        _phantom: PhantomData,
    }
}


/// The result of `err`
#[derive(Debug)]
#[must_use]
pub struct TxErr<Ctx, T, E> {
    err: E,
    _phantom: PhantomData<(Ctx, T)>,
}

impl<Ctx, T, E> Transaction for TxErr<Ctx, T, E>
where
    E: Clone,
{
    type Ctx = Ctx;
    type Item = T;
    type Err = E;
    fn run(&self, _ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        Err(self.err.clone())
    }
}
