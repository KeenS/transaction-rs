# 0.2.0 
* The type parameter `Ctx` of `Transaction` is now an associated type.
* Some methods of `Transaction` is changed to use `IntoTransaction` instead of `Transaction`
* `branchX` methods are added
* `IntoTransaction` is implemented for some Types like `Result` and tupules.
* `repeat`, `retry`, `loop_fn`, `join_all` is added

# 0.1.0
* first release
