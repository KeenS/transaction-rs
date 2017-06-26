# Requirements

* diesel_cli
* docker

# Building

``` console
$ docker-compose up -d
$ diesel database setup
$ cargo build
```

# Running

``` console
$ cargo run
created user: User { id: 1, name: "keen" }
updated user: User { id: 1, name: "KeenS" }
```
