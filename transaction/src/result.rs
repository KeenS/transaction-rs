use std::marker::PhantomData;

use Transaction;

/// The result of `result`
#[derive(Debug)]
#[must_use]
pub struct TxResult<Ctx, T, E> {
    r: Result<T, E>,
    _phantom: PhantomData<Ctx>,
}

/// Take a result and make a leaf transaction value.
pub fn result<Ctx, T, E>(r: Result<T, E>) -> TxResult<Ctx, T, E> {
    TxResult {
        r: r,
        _phantom: PhantomData,
    }
}

impl<Ctx, T, E> Transaction for TxResult<Ctx, T, E>
where
    T: Clone,
    E: Clone,
{
    type Ctx = Ctx;
    type Item = T;
    type Err = E;
    fn run(&self, _ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        self.r.clone()
    }
}
