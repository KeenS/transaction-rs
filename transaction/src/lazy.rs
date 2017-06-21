use std::marker::PhantomData;

use Transaction;

/// lazy evaluated transaction value.
/// Note that inner function can be called many times.
pub fn lazy<Ctx, F, T, E>(f: F) -> Lazy<Ctx, F>
where
    F: Fn() -> Result<T, E>,
{
    Lazy {
        f: f,
        _phantom: PhantomData,
    }
}

/// The result of `lazy`
#[derive(Debug)]
#[must_use]
pub struct Lazy<Ctx, F> {
    f: F,
    _phantom: PhantomData<Ctx>,
}

impl<Ctx, T, E, F> Transaction for Lazy<Ctx, F>
where
    F: Fn() -> Result<T, E>,
{
    type Ctx = Ctx;
    type Item = T;
    type Err = E;
    fn run(&self, _ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        (self.f)()
    }
}
