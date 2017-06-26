//! # Zero-cost transaction abstraction in Rust
//! This crate abstracts over transactions like STM, SQL transactions and so
//! on. It is composable via combinators and does DI of transactions.
//!
//!
//! The basic idea is representing contracts of "this computation must be run under a transaction"
//! as types. The trait `Transaction` represents a sequence of computation that
//! must be run under a transaction. And transactions are composable
//! (sequencable) using `then`, `and_then`, `or_else`, hence you can use it
//! like values wrapped in `Result`. Since it represents computation to be run
//! in data, some types respond to control operators are provided: `abort` for
//! `?`, `repeat` for `for`, `loop_fn` for `loop` and `branch` for (join point
//! of) `if` and so on. As all the combinators have its own result type, no
//! dispatches are done at execution time thus it is zero-cost.
//!
//! Another feature is it does DI of transaction. For database transaction, it
//! means that it injects DB connection from the context.
//!
//! # Examples
//!
//! ```
//!
//! extern crate transaction;
//!
//! use self::transaction::prelude::*;
//!
//! # struct FooConnection;
//! # struct FooError;
//! # #[derive(Clone)]struct User;
//!
//! // Since current rust doesn't support `impl Trait`, you need to make a
//! // trait box
//! // to return a trait value from a function.
//! type BoxTx<'a, T> = Box<Transaction<
//!                           Ctx = FooConnection,
//!                           Item = T,
//!                           Err =FooError>
//!                         + 'a>;
//!
//! fn find_user<'a>(id: i64) -> BoxTx<'a, Option<User>> {
//!     // connection is inejected from the context
//!     with_ctx(move |cn: &mut FooConnection| {
//!         // ..
//!         # let _ = (id, cn);
//!         # unimplemented!()
//!     }).boxed()
//!
//! }
//!
//! fn update_user<'a>(id: i64, name: &'a str) -> BoxTx<'a, Option<()>> {
//!     with_ctx(move |cn: &mut FooConnection| {
//!         // ..
//!         # let _ = (id, cn, name);
//!         # unimplemented!()
//!     }).boxed()
//! }
//!
//! fn update_find_user<'a>(id: i64, name: &'a str) -> BoxTx<'a, Option<User>> {
//!     update_user(id, name)
//!         // transaction can be composed using `and_then`
//!         .and_then(move |ret| match ret {
//!             None =>
//!                 // to return a leaf transaction, use `ok`, `err` or `result`
//!                 ok(None)
//!                 // to return from a branch (or, to match types at join
//!                 // point), use `branch` API
//!                 .branch()
//!                 // use `first` in the first arm of the brnach
//!                 .first(),
//!             Some(()) => find_user(id)
//!                 .branch()
//!                 // use `second` in the second arm of the brnach
//!                 .second(),
//!         })
//!         // finally, box it to return `BoxTx`.
//!         .boxed()
//! }
//! # fn main() {}
//! ```


#[cfg(feature = "mdo")]
pub mod mdo;

pub mod prelude {
    pub use super::Transaction;
    pub use err::err;
    pub use join_all::join_all;
    pub use lazy::lazy;
    pub use loop_fn::loop_fn;
    pub use ok::ok;
    pub use repeat::repeat;
    pub use result::result;
    pub use retry::retry;
    pub use with_ctx::with_ctx;
}

mod then;
mod map;
mod and_then;
mod map_err;
mod or_else;
mod abort;
mod try_abort;
mod recover;
mod try_recover;
mod join;
mod join3;
mod join4;
mod branch;
mod branch3;
mod branch4;
mod loop_fn;
mod repeat;
mod retry;
mod result;
mod ok;
mod err;
mod lazy;
mod join_all;
mod with_ctx;

pub use abort::*;
pub use and_then::*;
pub use branch::*;
pub use branch3::*;
pub use branch4::*;
pub use err::*;
pub use join::*;
pub use join3::*;
pub use join4::*;
pub use join_all::*;
pub use lazy::*;
pub use loop_fn::*;
pub use map::*;
pub use map_err::*;
pub use ok::*;
pub use or_else::*;
pub use recover::*;
pub use repeat::*;
pub use result::*;
pub use retry::*;
pub use then::*;
pub use try_abort::*;
pub use try_recover::*;
pub use with_ctx::*;

/// An abstract transaction. Transactions sharing the same `Ctx` can be
/// composed with combinators. When the transaction return an error, it means
/// the transaction is failed. Some runners may abort the transaction and the
/// other may retry the computation. Thus all the computation should be
/// idempotent (of cause, except operations using context). Note that this
/// transaction is not executed until it is `run`.
#[must_use]
pub trait Transaction {
    /// The contxt type (i.e. transaction type) of the transaction
    type Ctx;
    /// The return type of the transaction
    type Item;
    /// The error type of the transaction
    type Err;

    /// Run the transaction. This will called by transaction runner rather than
    /// user by hand.
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
        Tx2: IntoTransaction<Self::Ctx, Item = B, Err = Self::Err>,
        F: Fn(Result<Self::Item, Self::Err>) -> Tx2,
        Self: Sized,
    {
        then(self, f)
    }

    /// Transform the previous successful value
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Item) -> B,
        Self: Sized,
    {
        map(self, f)
    }



    /// Take the previous successful value of computation and do another
    /// computation
    fn and_then<F, B>(self, f: F) -> AndThen<Self, F, B>
    where
        B: IntoTransaction<Self::Ctx, Err = Self::Err>,
        F: Fn(Self::Item) -> B,
        Self: Sized,
    {
        and_then(self, f)
    }

    /// Transform the previous error value
    fn map_err<F, B>(self, f: F) -> MapErr<Self, F>
    where
        F: Fn(Self::Err) -> B,
        Self: Sized,
    {
        map_err(self, f)
    }


    /// Take the previous error value of computation and do another computation.
    /// This may be used falling back
    fn or_else<F, B>(self, f: F) -> OrElse<Self, F, B>
    where
        B: IntoTransaction<Self::Ctx, Item = Self::Item>,
        F: Fn(Self::Err) -> B,
        Self: Sized,
    {
        or_else(self, f)
    }

    /// Take the previous successfull value of computation and abort the
    /// transaction.
    fn abort<T, F>(self, f: F) -> Abort<Self, T, F>
    where
        F: Fn(Self::Item) -> Self::Err,
        Self: Sized,
    {
        abort(self, f)
    }

    /// Try to abort the transaction
    fn try_abort<F, B>(self, f: F) -> TryAbort<Self, F, B>
    where
        F: Fn(Self::Item) -> Result<B, Self::Err>,
        Self: Sized,
    {
        try_abort(self, f)
    }

    /// Recover from an error
    fn recover<T, F>(self, f: F) -> Recover<Self, T, F>
    where
        F: Fn(Self::Err) -> Self::Item,
        Self: Sized,
    {
        recover(self, f)
    }

    /// Try to recover from an error
    fn try_recover<F, B>(self, f: F) -> TryRecover<Self, F, B>
    where
        F: Fn(Self::Err) -> Result<Self::Item, B>,
        Self: Sized,
    {
        try_recover(self, f)
    }

    /// join 2 indepndant transactions
    fn join<B>(self, b: B) -> Join<Self, B::Tx>
    where
        B: IntoTransaction<Self::Ctx, Err = Self::Err>,
        Self: Sized,
    {
        join(self, b)
    }

    /// join 3 indepndant transactions
    fn join3<B, C>(self, b: B, c: C) -> Join3<Self, B::Tx, C::Tx>
    where
        B: IntoTransaction<Self::Ctx, Err = Self::Err>,
        C: IntoTransaction<Self::Ctx, Err = Self::Err>,
        Self: Sized,
    {
        join3(self, b, c)
    }

    /// join 4 indepndant transactions
    fn join4<B, C, D>(self, b: B, c: C, d: D) -> Join4<Self, B::Tx, C::Tx, D::Tx>
    where
        B: IntoTransaction<Self::Ctx, Err = Self::Err>,
        C: IntoTransaction<Self::Ctx, Err = Self::Err>,
        D: IntoTransaction<Self::Ctx, Err = Self::Err>,
        Self: Sized,
    {
        join4(self, b, c, d)
    }

    /// branch builder
    fn branch(self) -> BranchBuilder<Self>
    where
        Self: Sized,
    {
        BranchBuilder::new(self)
    }

    /// 3 branch builder
    fn branch3(self) -> Branch3Builder<Self>
    where
        Self: Sized,
    {
        Branch3Builder::new(self)
    }

    /// 4 branch builder
    fn branch4(self) -> Branch4Builder<Self>
    where
        Self: Sized,
    {
        Branch4Builder::new(self)
    }
}

/// types than can be converted into transaction
pub trait IntoTransaction<Ctx> {
    type Tx: Transaction<Ctx = Ctx, Item = Self::Item, Err = Self::Err>;
    type Err;
    type Item;

    fn into_transaction(self) -> Self::Tx;
}

impl<Tx, Ctx> IntoTransaction<Ctx> for Tx
where
    Tx: Transaction<Ctx = Ctx>,
{
    type Tx = Tx;
    type Err = Tx::Err;
    type Item = Tx::Item;

    fn into_transaction(self) -> Self::Tx {
        self
    }
}

impl<Ctx, Tx1, Tx2> IntoTransaction<Ctx> for (Tx1, Tx2)
where
    Tx1: IntoTransaction<Ctx>,
    Tx2: IntoTransaction<Ctx, Err = Tx1::Err>,
{
    type Tx = Join<Tx1::Tx, Tx2::Tx>;
    type Err = Tx1::Err;
    type Item = (Tx1::Item, Tx2::Item);
    fn into_transaction(self) -> Self::Tx {
        let (tx1, tx2) = self;
        tx1.into_transaction().join(tx2.into_transaction())
    }
}

impl<Ctx, Tx1, Tx2, Tx3> IntoTransaction<Ctx> for (Tx1, Tx2, Tx3)
where
    Tx1: IntoTransaction<Ctx>,
    Tx2: IntoTransaction<
        Ctx,
        Err = Tx1::Err,
    >,
    Tx3: IntoTransaction<
        Ctx,
        Err = Tx1::Err,
    >,
{
    type Tx = Join3<Tx1::Tx, Tx2::Tx, Tx3::Tx>;
    type Err = Tx1::Err;
    type Item = (Tx1::Item, Tx2::Item, Tx3::Item);
    fn into_transaction(self) -> Self::Tx {
        let (tx1, tx2, tx3) = self;
        tx1.into_transaction().join3(
            tx2.into_transaction(),
            tx3.into_transaction(),
        )
    }
}

impl<Ctx, Tx1, Tx2, Tx3, Tx4> IntoTransaction<Ctx> for (Tx1, Tx2, Tx3, Tx4)
where
    Tx1: IntoTransaction<Ctx>,
    Tx2: IntoTransaction<
        Ctx,
        Err = Tx1::Err,
    >,
    Tx3: IntoTransaction<
        Ctx,
        Err = Tx1::Err,
    >,
    Tx4: IntoTransaction<
        Ctx,
        Err = Tx1::Err,
    >,
{
    type Tx = Join4<Tx1::Tx, Tx2::Tx, Tx3::Tx, Tx4::Tx>;
    type Err = Tx1::Err;
    type Item = (Tx1::Item, Tx2::Item, Tx3::Item, Tx4::Item);
    fn into_transaction(self) -> Self::Tx {
        let (tx1, tx2, tx3, tx4) = self;
        tx1.into_transaction().join4(
            tx2.into_transaction(),
            tx3.into_transaction(),
            tx4.into_transaction(),
        )
    }
}

impl<Ctx, T, E> IntoTransaction<Ctx> for Result<T, E>
where
    T: Clone,
    E: Clone,
{
    type Tx = result::TxResult<Ctx, T, E>;
    type Err = E;
    type Item = T;

    fn into_transaction(self) -> Self::Tx {
        result::result(self)
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
