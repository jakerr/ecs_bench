#![feature(test)]

extern crate test;
use test::Bencher;

extern crate parsec;

extern crate ecs_bench;

use parsec::{Entity, Component, Scheduler, Storage, VecStorage};

use ecs_bench::parallel::{R, W1, W2, N};

struct RComp(R);
impl Component for RComp {
    type Storage = VecStorage<RComp>;
}

struct W1Comp(W1);
impl Component for W1Comp {
    type Storage = VecStorage<W1Comp>;
}

struct W2Comp(W2);
impl Component for W2Comp {
    type Storage = VecStorage<W2Comp>;
}

fn build() -> Scheduler {
    let mut w = parsec::World::new();
    w.register::<RComp>();
    w.register::<W1Comp>();
    w.register::<W2Comp>();

    // setup entities
    {
        let ents: Vec<Entity> = w.create_iter().take(N).collect();

        let mut rs = w.write::<RComp>();
        let mut w1s = w.write::<W1Comp>();
        let mut w2s = w.write::<W2Comp>();

        for e in ents {
            rs.insert(e, RComp(R { x: 0.0 }));
            w1s.insert(e, W1Comp(W1 { x: 0.0 }));
            w2s.insert(e, W2Comp(W2 { x: 0.0 }));
        }
    }

    Scheduler::new(w, 4)
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut scheduler = build();

    b.iter(|| {
        scheduler.run1w1r(|w1: &mut W1Comp, r: &RComp| w1.0.x += r.0.x);
        scheduler.run1w1r(|w2: &mut W2Comp, r: &RComp| w2.0.x *= r.0.x);
        scheduler.wait();
    });
}
