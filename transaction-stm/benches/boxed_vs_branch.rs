#![feature(test)]

extern crate stm;
extern crate transaction;
extern crate transaction_stm;
extern crate test;

use transaction::prelude::*;
use transaction_stm::{run, with_tx};
use test::Bencher;

type BoxTx<'a, T> = Box<Transaction<Ctx = stm::Transaction, Item = T, Err = stm::StmError> + 'a>;

fn inc<'a>(x: &'a stm::TVar<usize>) -> BoxTx<'a, usize> {
    with_tx(move |ctx| {
                let xv = ctx.read(&x)?;
                ctx.write(&x, xv + 1)?;
                Ok(xv)
            }).boxed()
}

fn halve<'a>(x: &'a stm::TVar<usize>) -> BoxTx<'a, usize> {
    with_tx(move |ctx| {
                let xv = ctx.read(&x)?;
                ctx.write(&x, xv / 2)?;
                Ok(xv)
            }).boxed()
}


#[bench]
fn bench_branch(b: &mut Bencher) {
    let x = stm::TVar::new(0);
    let tx = repeat(1_000, |i| if i % 2 == 0 {
        inc(&x).and_then(|_| halve(&x)).map(|_| ()).branch().first()
    } else {
        inc(&x).map(|_| ()).branch().second()

    });
    b.iter(|| run(&tx));
}

#[bench]
fn bench_boxed(b: &mut Bencher) {
    let x = stm::TVar::new(0);
    let tx = repeat(1_000, |i| if i % 2 == 0 {
        inc(&x).and_then(|_| halve(&x)).map(|_| ()).boxed()
    } else {
        inc(&x).map(|_| ()).boxed()

    });
    b.iter(|| run(&tx));
}

#[bench]
fn bench_nonboxing(b: &mut Bencher) {
    let x = stm::TVar::new(0);
    let tx = repeat(1_000, |i| if i % 2 == 0 {
        with_tx(|ctx| {
                    let xv = ctx.read(&x)?;
                    ctx.write(&x, xv + 1)?;
                    Ok(xv)
                }).and_then(|_| {
                          with_tx(|ctx| {
                                      let xv = ctx.read(&x)?;
                                      ctx.write(&x, xv / 2)?;
                                      Ok(xv)
                                  })
                      })
            .map(|_| ())
            .branch()
            .first()
    } else {
        with_tx(|ctx| {
                    let xv = ctx.read(&x)?;
                    ctx.write(&x, xv + 1)?;
                    Ok(xv)
                }).map(|_| ())
            .branch()
            .second()

    });
    b.iter(|| run(&tx));
}


// running 3 tests
// test bench_boxed     ... bench:     202,656 ns/iter (+/- 7,825)
// test bench_branch    ... bench:     188,261 ns/iter (+/- 13,502)
// test bench_nonboxing ... bench:     176,353 ns/iter (+/- 13,134)

// test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out
