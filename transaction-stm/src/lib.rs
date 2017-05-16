extern crate stm;
extern crate transaction;

use transaction::Transaction;
use stm::Transaction as Stm;


pub fn run<T, Tx>(tx: Tx) -> T
    where Tx: Transaction<Stm, Item = T, Err = stm::StmError>
{
    Stm::with(|stm| tx.run(stm))
}
