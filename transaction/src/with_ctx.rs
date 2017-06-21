use std::marker::PhantomData;

use Transaction;


/// Receive the context from the executing transaction and perform computation.
pub fn with_ctx<Ctx, F, T, E>(f: F) -> WithCtx<Ctx, F>
where
    F: Fn(&mut Ctx) -> Result<T, E>,
{
    WithCtx {
        f: f,
        _phantom: PhantomData,
    }
}

/// The result of `with_ctx`
#[derive(Debug)]
#[must_use]
pub struct WithCtx<Ctx, F> {
    f: F,
    _phantom: PhantomData<Ctx>,
}

impl<Ctx, T, E, F> Transaction for WithCtx<Ctx, F>
where
    F: Fn(&mut Ctx) -> Result<T, E>,
{
    type Ctx = Ctx;
    type Item = T;
    type Err = E;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        (self.f)(ctx)
    }
}
