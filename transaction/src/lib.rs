//! # Zero-cost transactions in Rust
//! This crate abstracts over transactions like STM, SQL transactions and so on.
//! It is also composable via combinators and do DI of transactions.


use std::marker::PhantomData;

#[cfg(feature = "mdo")]
pub mod mdo;

pub mod prelude {
    pub use super::{Transaction, result, ok, join_vec, err, lazy, with_ctx, repeat};
}

/// An abstract transaction.
/// Transactions sharing the same `Ctx` can be composed with combinators.
#[must_use]
pub trait Transaction {
    type Ctx;
    /// The return type of the transaction
    type Item;
    /// The error type of the transaction
    type Err;

    /// Run the transaction. This will called by transaction runner rather than user by hand.
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err>;

    /// Box the transaction
    fn boxed<'a>(self) -> Box<Transaction<Ctx = Self::Ctx, Item = Self::Item, Err = Self::Err> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }

    /// Take the previous result of computation and do another computation
    fn then<F, B, Tx2>(self, f: F) -> Then<Self, F, Tx2>
    where
        Tx2: Transaction<Ctx = Self::Ctx, Item = B, Err = Self::Err>,
        F: Fn(Result<Self::Item, Self::Err>) -> Tx2,
        Self: Sized,
    {
        Then {
            tx: self,
            f: f,
            _phantom: PhantomData,
        }
    }

    /// transform the previous successful value
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Item) -> B,
        Self: Sized,
    {
        Map { tx: self, f: f }
    }



    /// Take the previous successful value of computation and do another computation
    fn and_then<F, B>(self, f: F) -> AndThen<Self, F, B>
    where
        B: Transaction<Ctx = Self::Ctx, Err = Self::Err>,
        F: Fn(Self::Item) -> B,
        Self: Sized,
    {
        AndThen {
            tx: self,
            f: f,
            _phantom: PhantomData,
        }
    }

    /// transform the previous error value
    fn map_err<F, B>(self, f: F) -> MapErr<Self, F>
    where
        F: Fn(Self::Err) -> B,
        Self: Sized,
    {
        MapErr { tx: self, f: f }
    }


    /// Take the previous error value of computation and do another computation.
    /// This may be used falling back
    fn or_else<F, B>(self, f: F) -> OrElse<Self, F, B>
    where
        B: Transaction<Ctx = Self::Ctx, Item = Self::Item>,
        F: Fn(Self::Err) -> B,
        Self: Sized,
    {
        OrElse {
            tx: self,
            f: f,
            _phantom: PhantomData,
        }
    }

    /// Abort the transaction
    fn abort<T, F>(self, f: F) -> Abort<Self, T, F>
    where
        F: Fn(Self::Item) -> Self::Err,
        Self: Sized,
    {
        Abort {
            tx: self,
            f: f,
            _phantom: PhantomData,
        }
    }

    /// Try to abort the transaction
    fn try_abort<F, B>(self, f: F) -> TryAbort<Self, F, B>
    where
        F: Fn(Self::Item) -> Result<B, Self::Err>,
        Self: Sized,
    {
        TryAbort {
            tx: self,
            f: f,
            _phantom: PhantomData,
        }
    }

    /// Recover the transaction
    fn recover<T, F>(self, f: F) -> Recover<Self, T, F>
    where
        F: Fn(Self::Item) -> Self::Err,
        Self: Sized,
    {
        Recover {
            tx: self,
            f: f,
            _phantom: PhantomData,
        }
    }

    /// Try to recover the transaction
    fn try_recover<F, B>(self, f: F) -> TryRecover<Self, F, B>
    where
        F: Fn(Self::Item) -> Result<B, Self::Err>,
        Self: Sized,
    {
        TryRecover {
            tx: self,
            f: f,
            _phantom: PhantomData,
        }
    }

    /// join 2 indepndant transactions
    fn join<B>(self, b: B) -> Join<Self, B>
    where
        B: Transaction<Ctx = Self::Ctx, Err = Self::Err>,
        Self: Sized,
    {
        Join { tx1: self, tx2: b }
    }

    /// join 3 indepndant transactions
    fn join3<B, C>(self, b: B, c: C) -> Join3<Self, B, C>
    where
        B: Transaction<Ctx = Self::Ctx, Err = Self::Err>,
        C: Transaction<Ctx = Self::Ctx, Err = Self::Err>,
        Self: Sized,
    {
        Join3 {
            tx1: self,
            tx2: b,
            tx3: c,
        }
    }

    /// join 4 indepndant transactions
    fn join4<B, C, D>(self, b: B, c: C, d: D) -> Join4<Self, B, C, D>
    where
        B: Transaction<Ctx = Self::Ctx, Err = Self::Err>,
        C: Transaction<Ctx = Self::Ctx, Err = Self::Err>,
        D: Transaction<Ctx = Self::Ctx, Err = Self::Err>,
        Self: Sized,
    {
        Join4 {
            tx1: self,
            tx2: b,
            tx3: c,
            tx4: d,
        }
    }

    /// branch builder
    fn branch(self) -> BranchBuilder<Self>
    where
        Self: Sized,
    {
        BranchBuilder(self)
    }

    /// 3 branch builder
    fn branch3(self) -> Branch3Builder<Self>
    where
        Self: Sized,
    {
        Branch3Builder(self)
    }

    /// 4 branch builder
    fn branch4(self) -> Branch4Builder<Self>
    where
        Self: Sized,
    {
        Branch4Builder(self)
    }

    // retry
}

/// Not used for now.
pub trait IntoTransaction {
    type Tx: Transaction<Ctx = Self::Ctx, Item = Self::Item, Err = Self::Err>;
    type Ctx;
    type Err;
    type Item;

    fn into_transaction(self) -> Self::Tx;
}


/// Take a result and make a leaf transaction value.
pub fn result<Ctx, T, E>(r: Result<T, E>) -> TxResult<Ctx, T, E> {
    TxResult {
        r: r,
        _phantom: PhantomData,
    }
}

/// make a successful transaction value.
pub fn ok<Ctx, T, E>(t: T) -> TxOk<Ctx, T, E> {
    TxOk {
        ok: t,
        _phantom: PhantomData,
    }
}

/// make a error transaction value.
pub fn err<Ctx, T, E>(e: E) -> TxErr<Ctx, T, E> {
    TxErr {
        err: e,
        _phantom: PhantomData,
    }
}

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

/// join a vec of transaction
pub fn join_vec<B>(vec: Vec<B>) -> JoinVec<B>
where
    B: Transaction,
{
    JoinVec { vec: vec }
}

pub fn repeat<F, Tx>(n: usize, f: F) -> Repeat<F, Tx>
where
    Tx: Transaction,
    F: Fn(usize) -> Tx,
{
    Repeat {
        n: n,
        f: f,
        _phantom: PhantomData,
    }
}


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

/// The result of `then`
#[derive(Debug)]
#[must_use]
pub struct Then<Tx1, F, Tx2> {
    tx: Tx1,
    f: F,
    _phantom: PhantomData<Tx2>,
}

/// The result of `map`
#[derive(Debug)]
#[must_use]
pub struct Map<Tx, F> {
    tx: Tx,
    f: F,
}


/// The result of `and_then`
#[derive(Debug)]
#[must_use]
pub struct AndThen<Tx1, F, Tx2> {
    tx: Tx1,
    f: F,
    _phantom: PhantomData<Tx2>,
}


/// The result of `map_err`
#[derive(Debug)]
#[must_use]
pub struct MapErr<Tx, F> {
    tx: Tx,
    f: F,
}

/// The result of `or_else`
#[derive(Debug)]
#[must_use]
pub struct OrElse<Tx1, F, Tx2> {
    tx: Tx1,
    f: F,
    _phantom: PhantomData<Tx2>,
}

/// The result of `abort`
#[derive(Debug)]
#[must_use]
pub struct Abort<Tx, T, F> {
    tx: Tx,
    f: F,
    _phantom: PhantomData<T>,
}

/// The result of `try_abort`
#[derive(Debug)]
#[must_use]
pub struct TryAbort<Tx, F, B> {
    tx: Tx,
    f: F,
    _phantom: PhantomData<B>,
}


/// The result of `recover`
#[derive(Debug)]
#[must_use]
pub struct Recover<Tx, T, F> {
    tx: Tx,
    f: F,
    _phantom: PhantomData<T>,
}

/// The result of `try_recover`
#[derive(Debug)]
#[must_use]
pub struct TryRecover<Tx, F, B> {
    tx: Tx,
    f: F,
    _phantom: PhantomData<B>,
}

/// The result of `join`
#[derive(Debug)]
#[must_use]
pub struct Join<Tx1, Tx2> {
    tx1: Tx1,
    tx2: Tx2,
}

/// The result of `join3`
#[derive(Debug)]
#[must_use]
pub struct Join3<Tx1, Tx2, Tx3> {
    tx1: Tx1,
    tx2: Tx2,
    tx3: Tx3,
}

/// The result of `join4`
#[derive(Debug)]
#[must_use]
pub struct Join4<Tx1, Tx2, Tx3, Tx4> {
    tx1: Tx1,
    tx2: Tx2,
    tx3: Tx3,
    tx4: Tx4,
}

/// BranchBuilder
#[derive(Debug)]
#[must_use]
pub struct BranchBuilder<Tx>(Tx);

impl<Tx> BranchBuilder<Tx> {
    pub fn first<B>(self) -> Branch<Tx, B> {
        Branch::B1(self.0)
    }

    pub fn second<B>(self) -> Branch<B, Tx> {
        Branch::B2(self.0)
    }
}

/// The result of `branch`
#[derive(Debug)]
#[must_use]
pub enum Branch<Tx1, Tx2> {
    B1(Tx1),
    B2(Tx2),
}

/// Branch3Builder
#[derive(Debug)]
#[must_use]
pub struct Branch3Builder<Tx>(Tx);

impl<Tx> Branch3Builder<Tx> {
    pub fn first<B, C>(self) -> Branch3<Tx, B, C> {
        Branch3::B1(self.0)
    }

    pub fn second<B, C>(self) -> Branch3<B, Tx, C> {
        Branch3::B2(self.0)
    }

    pub fn third<B, C>(self) -> Branch3<B, C, Tx> {
        Branch3::B3(self.0)
    }
}


/// The result of `branch3`
#[derive(Debug)]
#[must_use]
pub enum Branch3<Tx1, Tx2, Tx3> {
    B1(Tx1),
    B2(Tx2),
    B3(Tx3),
}

/// Branch4Builder
#[derive(Debug)]
#[must_use]
pub struct Branch4Builder<Tx>(Tx);

impl<Tx> Branch4Builder<Tx> {
    pub fn first<B, C, D>(self) -> Branch4<Tx, B, C, D> {
        Branch4::B1(self.0)
    }

    pub fn second<B, C, D>(self) -> Branch4<B, Tx, C, D> {
        Branch4::B2(self.0)
    }

    pub fn third<B, C, D>(self) -> Branch4<B, C, Tx, D> {
        Branch4::B3(self.0)
    }

    pub fn fourth<B, C, D>(self) -> Branch4<B, C, D, Tx> {
        Branch4::B4(self.0)
    }
}


/// The result of `branch4`
#[derive(Debug)]
#[must_use]
pub enum Branch4<Tx1, Tx2, Tx3, Tx4> {
    B1(Tx1),
    B2(Tx2),
    B3(Tx3),
    B4(Tx4),
}

/// The result of `repeat`
#[derive(Debug)]
#[must_use]
pub struct Repeat<F, Tx> {
    n: usize,
    f: F,
    _phantom: PhantomData<Tx>,
}

/// The result of `result`
#[derive(Debug)]
#[must_use]
pub struct TxResult<Ctx, T, E> {
    r: Result<T, E>,
    _phantom: PhantomData<Ctx>,
}

/// The result of `ok`
#[derive(Debug)]
#[must_use]
pub struct TxOk<Ctx, T, E> {
    ok: T,
    _phantom: PhantomData<(Ctx, E)>,
}

/// The result of `err`
#[derive(Debug)]
#[must_use]
pub struct TxErr<Ctx, T, E> {
    err: E,
    _phantom: PhantomData<(Ctx, T)>,
}

/// The result of `lazy`
#[derive(Debug)]
#[must_use]
pub struct Lazy<Ctx, F> {
    f: F,
    _phantom: PhantomData<Ctx>,
}

/// The result of `join_vec`
#[derive(Debug)]
#[must_use]
pub struct JoinVec<Tx> {
    vec: Vec<Tx>,
}

/// The result of `with_ctx`
#[derive(Debug)]
#[must_use]
pub struct WithCtx<Ctx, F> {
    f: F,
    _phantom: PhantomData<Ctx>,
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

impl<Tx, Tx2, F> Transaction for Then<Tx, F, Tx2>
where
    Tx2: Transaction<Ctx = Tx::Ctx, Err = Tx::Err>,
    Tx: Transaction,
    F: Fn(Result<Tx::Item, Tx::Err>) -> Tx2,
{
    type Ctx = Tx::Ctx;
    type Item = Tx2::Item;
    type Err = Tx2::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Then { ref tx, ref f, .. } = self;
        f(tx.run(ctx)).run(ctx)
    }
}


impl<Tx, Tx2, F> Transaction for AndThen<Tx, F, Tx2>
where
    Tx2: Transaction<Ctx = Tx::Ctx, Err = Tx::Err>,
    Tx: Transaction,
    F: Fn(Tx::Item) -> Tx2,
{
    type Ctx = Tx::Ctx;
    type Item = Tx2::Item;
    type Err = Tx2::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &AndThen { ref tx, ref f, .. } = self;
        tx.run(ctx).and_then(|item| f(item).run(ctx))
    }
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


impl<Tx, Tx2, F> Transaction for OrElse<Tx, F, Tx2>
where
    Tx2: Transaction<Ctx = Tx::Ctx, Item = Tx::Item>,
    Tx: Transaction,
    F: Fn(Tx::Err) -> Tx2,
{
    type Ctx = Tx2::Ctx;
    type Item = Tx2::Item;
    type Err = Tx2::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &OrElse { ref tx, ref f, .. } = self;
        tx.run(ctx).or_else(|item| f(item).run(ctx))
    }
}


impl<Tx, F, T> Transaction for Abort<Tx, T, F>
where
    Tx: Transaction,
    F: Fn(Tx::Item) -> Tx::Err,
{
    type Ctx = Tx::Ctx;
    type Item = T;
    type Err = Tx::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Abort { ref tx, ref f, .. } = self;
        match tx.run(ctx) {
            Ok(r) => Err(f(r)),
            Err(e) => Err(e),
        }
    }
}

impl<Tx, F, B> Transaction for TryAbort<Tx, F, B>
where
    Tx: Transaction,
    F: Fn(Tx::Item) -> Result<B, Tx::Err>,
{
    type Ctx = Tx::Ctx;
    type Item = B;
    type Err = Tx::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &TryAbort { ref tx, ref f, .. } = self;
        match tx.run(ctx) {
            Ok(r) => f(r),
            Err(e) => Err(e),
        }
    }
}



impl<Tx, F, T> Transaction for Recover<Tx, T, F>
where
    Tx: Transaction,
    F: Fn(Tx::Err) -> Tx::Item,
{
    type Ctx = Tx::Ctx;
    type Item = Tx::Item;
    type Err = Tx::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Recover { ref tx, ref f, .. } = self;
        match tx.run(ctx) {
            r @ Ok(_) => r,
            Err(e) => Ok(f(e)),
        }
    }
}

impl<Tx, F, B> Transaction for TryRecover<Tx, F, B>
where
    Tx: Transaction,
    F: Fn(Tx::Err) -> Result<Tx::Item, B>,
{
    type Ctx = Tx::Ctx;
    type Item = Tx::Item;
    type Err = B;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &TryRecover { ref tx, ref f, .. } = self;
        match tx.run(ctx) {
            Ok(r) => Ok(r),
            Err(e) => f(e),
        }
    }
}

impl<Tx1, Tx2> Transaction for Join<Tx1, Tx2>
where
    Tx1: Transaction,
    Tx2: Transaction<Ctx = Tx1::Ctx, Err = Tx1::Err>,
{
    type Ctx = Tx1::Ctx;
    type Item = (Tx1::Item, Tx2::Item);
    type Err = Tx1::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Join { ref tx1, ref tx2 } = self;
        match (tx1.run(ctx), tx2.run(ctx)) {
            (Ok(r1), Ok(r2)) => Ok((r1, r2)),
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}

impl<Tx1, Tx2, Tx3> Transaction for Join3<Tx1, Tx2, Tx3>
    where Tx1: Transaction,
          Tx2: Transaction<Ctx = Tx1::Ctx, Err = Tx1::Err>,
          Tx3: Transaction<Ctx = Tx1::Ctx, Err = Tx1::Err>
{
    type Ctx = Tx1::Ctx;
    type Item = (Tx1::Item, Tx2::Item, Tx3::Item);
    type Err = Tx1::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Join3 {
            ref tx1,
            ref tx2,
            ref tx3,
        } = self;
        match (tx1.run(ctx), tx2.run(ctx), tx3.run(ctx)) {
            (Ok(r1), Ok(r2), Ok(r3)) => Ok((r1, r2, r3)),
            (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) => Err(e),
        }
    }
}

impl<Tx1, Tx2, Tx3, Tx4> Transaction for Join4<Tx1, Tx2, Tx3, Tx4>
where
    Tx1: Transaction,
    Tx2: Transaction<
        Ctx = Tx1::Ctx,
        Err = Tx1::Err,
    >,
    Tx3: Transaction<
        Ctx = Tx1::Ctx,
        Err = Tx1::Err,
    >,
    Tx4: Transaction<
        Ctx = Tx1::Ctx,
        Err = Tx1::Err,
    >,
{
    type Ctx = Tx1::Ctx;
    type Item = (Tx1::Item, Tx2::Item, Tx3::Item, Tx4::Item);
    type Err = Tx1::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let &Join4 {
            ref tx1,
            ref tx2,
            ref tx3,
            ref tx4,
        } = self;
        match (tx1.run(ctx), tx2.run(ctx), tx3.run(ctx), tx4.run(ctx)) {
            (Ok(r1), Ok(r2), Ok(r3), Ok(r4)) => Ok((r1, r2, r3, r4)),
            (Err(e), _, _, _) |
            (_, Err(e), _, _) |
            (_, _, Err(e), _) |
            (_, _, _, Err(e)) => Err(e),
        }
    }
}

impl<Tx1, Tx2> Transaction for Branch<Tx1, Tx2>
where
    Tx1: Transaction,
    Tx2: Transaction<
        Ctx = Tx1::Ctx,
        Item = Tx1::Item,
        Err = Tx1::Err,
    >,
{
    type Ctx = Tx1::Ctx;
    type Item = Tx1::Item;
    type Err = Tx1::Err;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        match *self {
            Branch::B1(ref tx) => tx.run(ctx),
            Branch::B2(ref tx) => tx.run(ctx),
        }
    }
}

impl<Tx1, Tx2, Tx3> Transaction for Branch3<Tx1, Tx2, Tx3>
where
    Tx1: Transaction,
    Tx2: Transaction<
        Ctx = Tx1::Ctx,
        Item = Tx1::Item,
        Err = Tx1::Err,
    >,
    Tx3: Transaction<
        Ctx = Tx1::Ctx,
        Item = Tx1::Item,
        Err = Tx1::Err,
    >,
{
    type Ctx = Tx1::Ctx;
    type Item = Tx1::Item;
    type Err = Tx1::Err;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        match *self {
            Branch3::B1(ref tx) => tx.run(ctx),
            Branch3::B2(ref tx) => tx.run(ctx),
            Branch3::B3(ref tx) => tx.run(ctx),
        }
    }
}

impl<Tx1, Tx2, Tx3, Tx4> Transaction for Branch4<Tx1, Tx2, Tx3, Tx4>
where
    Tx1: Transaction,
    Tx2: Transaction<
        Ctx = Tx1::Ctx,
        Item = Tx1::Item,
        Err = Tx1::Err,
    >,
    Tx3: Transaction<
        Ctx = Tx1::Ctx,
        Item = Tx1::Item,
        Err = Tx1::Err,
    >,
    Tx4: Transaction<
        Ctx = Tx1::Ctx,
        Item = Tx1::Item,
        Err = Tx1::Err,
    >,
{
    type Ctx = Tx1::Ctx;
    type Item = Tx1::Item;
    type Err = Tx1::Err;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        match *self {
            Branch4::B1(ref tx) => tx.run(ctx),
            Branch4::B2(ref tx) => tx.run(ctx),
            Branch4::B3(ref tx) => tx.run(ctx),
            Branch4::B4(ref tx) => tx.run(ctx),
        }
    }
}


impl<F, Tx> Transaction for Repeat<F, Tx>
where
    F: Fn(usize) -> Tx,
    Tx: Transaction,
{
    type Ctx = Tx::Ctx;
    type Item = Vec<Tx::Item>;
    type Err = Tx::Err;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let Repeat { ref n, ref f, .. } = *self;
        let mut ret = Vec::new();
        for i in 0..*n {
            ret.push(f(i).run(ctx)?)
        }
        Ok(ret)
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

impl<Tx> Transaction for JoinVec<Tx>
where
    Tx: Transaction,
{
    type Ctx = Tx::Ctx;
    type Item = Vec<Tx::Item>;
    type Err = Tx::Err;

    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        let vec = &self.vec;

        vec.iter()
            .map(|tx| tx.run(ctx))
            .collect::<Result<Vec<_>, _>>()
    }
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

impl<Ctx, T, E> Transaction for Fn(&mut Ctx) -> Result<T, E> {
    type Ctx = Ctx;
    type Item = T;
    type Err = E;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        self(ctx)
    }
}


impl<T> Transaction for Box<T>
where
    T: ?Sized + Transaction,
{
    type Ctx = T::Ctx;
    type Item = T::Item;
    type Err = T::Err;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        (**self).run(ctx)
    }
}

impl<'a, T> Transaction for &'a T
where
    T: ?Sized + Transaction,
{
    type Ctx = T::Ctx;
    type Item = T::Item;
    type Err = T::Err;
    fn run(&self, ctx: &mut Self::Ctx) -> Result<Self::Item, Self::Err> {
        (**self).run(ctx)
    }
}
