use Transaction;

/// BranchBuilder
#[derive(Debug)]
#[must_use]
pub struct BranchBuilder<Tx>(Tx);

impl<Tx> BranchBuilder<Tx> {
    pub fn new(tx: Tx) -> Self {
        BranchBuilder(tx)
    }

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
