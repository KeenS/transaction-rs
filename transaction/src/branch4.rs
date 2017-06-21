use Transaction;

/// Branch4Builder
#[derive(Debug)]
#[must_use]
pub struct Branch4Builder<Tx>(Tx);

impl<Tx> Branch4Builder<Tx> {
    pub fn new(tx: Tx) -> Self {
        Branch4Builder(tx)
    }

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
