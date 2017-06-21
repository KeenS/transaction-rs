use {IntoTransaction, Transaction};

/// join a vec of transaction
pub fn join_vec<Ctx, B>(vec: Vec<B>) -> JoinVec<B::Tx>
where
    B: IntoTransaction<Ctx>,
{
    JoinVec {
        vec: vec.into_iter()
            .map(IntoTransaction::into_transaction)
            .collect(),
    }
}

/// The result of `join_vec`
#[derive(Debug)]
#[must_use]
pub struct JoinVec<Tx> {
    vec: Vec<Tx>,
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
