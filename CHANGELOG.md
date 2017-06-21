# 0.2.0 2017-06-21

## transaction

* [break] The type parameter `Ctx` of `Transaction` is now an associated type.
* Some methods of `Transaction` is changed to use `IntoTransaction` instead of `Transaction`
* `branchX` methods are added
* `IntoTransaction` is implemented for some Types like `Result` and tupules.
* `repeat`, `retry`, `loop_fn`, `join_all` is added

## transaction-diesel

* catchup transaction update
* update diesel dependency

# 0.1.0 2017-06-06
* first release
