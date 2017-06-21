use Transaction;

/// Branch3Builder
#[derive(Debug)]
#[must_use]
pub struct Branch3Builder<Tx>(Tx);

impl<Tx> Branch3Builder<Tx> {
    pub fn new(tx: Tx) -> Self {
        Branch3Builder(tx)
    }

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
