# Transaction

An zero cost transaction abstraction library.
This crate provide the ways to abstract and compinate transactions.
Combinated comptations are run under a transaction.
Not only it can be composed run under a transaction, it also *requires* computations are composed and run under a transaction.

To run the transactions, use crates like [`transaction-stm`](../transaction-stm) or [`transaction-diesel`](../transaction-diesel)
