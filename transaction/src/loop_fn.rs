use std::marker::PhantomData;

use {IntoTransaction, Transaction};

pub fn loop_fn<Ctx, S, T, F, A>(initial_state: S, f: F) -> LoopFn<Ctx, F, A>
where
    A: IntoTransaction<Ctx, Item = Loop<S, T>>,
    F: Fn(S) -> A,
{
    LoopFn {
        tx: f(initial_state).into_transaction(),
        f: f,
        _phantom: PhantomData,
    }
}

/// The result of `loop_fn`
#[derive(Debug)]
#[must_use]
pub struct LoopFn<Ctx, F, A: IntoTransaction<Ctx>> {
    tx: A::Tx,
    f: F,
    _phantom: PhantomData<(Ctx)>,
}

/// The status of a `loop_fn` loop.
#[derive(Debug)]
pub enum Loop<S, T> {
    /// Indicates that the loop has completed with output `T`.
    Break(T),
    /// Indicates that the loop function should be called again with input state `S`.
    Continue(S),
}

impl<Ctx, S, T, F, A> Transaction for LoopFn<Ctx, F, A>
where
    F: Fn(S) -> A,
    A: IntoTransaction<Ctx, Item = Loop<S, T>>,
{
    type Ctx = Ctx;
    type Item = T;
    type Err = A::Err;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let LoopFn { ref tx, ref f, .. } = *self;
        let mut ret = tx.run(ctx)?;
        loop {
            let s = match ret {
                Loop::Break(t) => return Ok(t),
                Loop::Continue(s) => s,
            };
            ret = f(s).into_transaction().run(ctx)?;
        }
    }
}
