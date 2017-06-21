use std::marker::PhantomData;

use Transaction;

/// make a successful transaction value.
pub fn ok<Ctx, T, E>(t: T) -> TxOk<Ctx, T, E> {
    TxOk {
        ok: t,
        _phantom: PhantomData,
    }
}

/// The result of `ok`
#[derive(Debug)]
#[must_use]
pub struct TxOk<Ctx, T, E> {
    ok: T,
    _phantom: PhantomData<(Ctx, E)>,
}

impl<Ctx, T, E> Transaction for TxOk<Ctx, T, E>
where
    T: Clone,
{
    type Ctx = Ctx;
    type Item = T;
    type Err = E;
    fn run(&self, _ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        Ok(self.ok.clone())
    }
}
