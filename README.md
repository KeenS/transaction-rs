# transaction-rs
The transaction abstraction library and its executors.

This crate abstracts over transactions like STM, SQL transactions and so on. It is composable via combinators and does DI of transactions.

The basic idea is representing contracts of "this computation must be run under a transaction" as types. The trait `Transaction` represents a sequence of computation that must be run under a transaction. And transactions are composable (sequencable) using `then`, `and_then`, `or_else`, hence you can use it like values wrapped in `Result`. Since it represents computation to be run in data, some types respond to control operators are provided: `abort` for `?`, `repeat` for `for`, `loop_fn` for `loop` and `branch` for (join point of) `if` and so on. As all the combinators have its own result type, no dispatches are done at execution time thus it is zero-cost.

Another feature is it does DI of transaction. For database transaction, it means that it injects DB connection from the context.


See [transaction-stm/examples](transaction-stm/examples) or [transaction-diesel/examples](transaction-diesel/examples) for usage.


# Documentatins

* [transaction](https://docs.rs/transaction)
* [transaction-diesel](https://docs.rs/transaction-diesel)
* [transaction-stm](https://docs.rs/transaction-stm)
