use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rangetree::RangeTree;

fn bench_rangetree(c: &mut Criterion) {
    let mut tree = RangeTree::new();
    for i in 0..1000 {
        tree.insert((i * 200)..(i * 200 + 100), i);
    }

    c.bench_function("insert non-overlapping", |b| {
        b.iter_batched_ref(
            || tree.clone(),
            |tree| tree.insert(500..600, 999),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("insert overlapping end", |b| {
        b.iter_batched_ref(
            || tree.clone(),
            |tree| tree.insert(550..650, 999),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("insert overlapping start", |b| {
        b.iter_batched_ref(
            || tree.clone(),
            |tree| tree.insert(450..550, 999),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("insert overlapping all", |b| {
        b.iter_batched_ref(
            || tree.clone(),
            |tree| tree.insert(0..1000, 999),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("insert within existing", |b| {
        b.iter_batched_ref(
            || tree.clone(),
            |tree| tree.insert(120..180, 999),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("remove non-overlapping", |b| {
        b.iter_batched_ref(
            || tree.clone(),
            |tree| tree.remove(500..600),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("remove overlapping end", |b| {
        b.iter_batched_ref(
            || tree.clone(),
            |tree| tree.remove(550..650),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("remove overlapping start", |b| {
        b.iter_batched_ref(
            || tree.clone(),
            |tree| tree.remove(450..550),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("remove overlapping all", |b| {
        b.iter_batched_ref(
            || tree.clone(),
            |tree| tree.remove(0..1000),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("remove within existing", |b| {
        b.iter_batched_ref(
            || tree.clone(),
            |tree| tree.remove(120..180),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_rangetree);
criterion_main!(benches);
