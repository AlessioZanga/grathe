use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use grathe::prelude::*;
use grathe::storages::AdjacencyList;

const N: [usize; 14] = [
    0, 1, 2, 5, 10, 25, 50, 100, 500, 1_000, 5000, 10_000, 100_000, 1_000_000,
];

macro_rules! format_group_template {
    ($T:ident, $U:ident, $name:expr) => {
        paste::item! {
            format!("{}/{}", stringify!([< $T:lower _ $U:lower >]), $name)
        }
    };
}

macro_rules! criterion_storage_group_template {
    ($T:ident, $U:ident) => {
        paste::item! {
            fn [< $T:lower _ $U:lower _ new >] (c: &mut Criterion) {
                let group = format_group_template!($T, $U, "new");
                let mut group = c.benchmark_group(group);
                group.significance_level(0.01).sample_size(500);
                group.bench_function("new", |b| {
                    b.iter(|| {
                        black_box($T::<$U>::new());
                    })
                });
            }

            fn [< $T:lower _ $U:lower _ with_capacity >] (c: &mut Criterion) {
                let group = format_group_template!($T, $U, "with_capacity");
                let mut group = c.benchmark_group(group);
                group.significance_level(0.01).sample_size(500);
                for i in N {
                    group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
                        b.iter(|| {
                            $T::<$U>::with_capacity(black_box(i));
                        })
                    });
                }
                group.finish();
            }

            fn [< $T:lower _ $U:lower _ reserve >] (c: &mut Criterion) {
                let mut g = $T::<$U>::new();

                let group = format_group_template!($T, $U, "reserve");
                let mut group = c.benchmark_group(group);
                for i in N {
                    group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
                        b.iter(|| {
                            g.reserve(black_box(i));
                        })
                    });
                }
                group.finish();
            }

            fn [< $T:lower _ $U:lower _ from_order >] (c: &mut Criterion) {
                let group = format_group_template!($T, $U, "from_order");
                let mut group = c.benchmark_group(group);
                for i in N {
                    group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, &i| {
                        b.iter(|| {
                            $T::<$U>::from_order(black_box(i));
                        })
                    });
                }
            }

            fn [< $T:lower _ $U:lower _ from_vertices >] (c: &mut Criterion) {
                let v: Vec<(usize, Vec<$U>)> = N.iter().map(|&x| (x, (0..x as $U).collect())).collect();

                let group = format_group_template!($T, $U, "from_vertices");
                let mut group = c.benchmark_group(group);
                for (n, i) in &v {
                    group.bench_with_input(BenchmarkId::from_parameter(n), &i, |b, &i| {
                        b.iter(|| {
                            $T::<$U>::from_vertices(black_box(i));
                        })
                    });
                }
            }

            fn [< $T:lower _ $U:lower _ from_edges >] (c: &mut Criterion) {
                let e: Vec<(usize, Vec<($U, $U)>)> = N
                    .iter()
                    .map(|&x| (x, (0..x as $U).zip((0..x as $U).into_iter().rev()).collect()))
                    .collect();

                let group = format_group_template!($T, $U, "from_edges");
                let mut group = c.benchmark_group(group);
                for (n, i) in &e {
                    group.bench_with_input(BenchmarkId::from_parameter(n), &i, |b, &i| {
                        b.iter(|| {
                            $T::<$U>::from_edges(black_box(i));
                        })
                    });
                }
            }

            fn [< $T:lower _ $U:lower _ vertices_iter >] (c: &mut Criterion) {
                let gs: Vec<(usize, $T<$U>)> = N.iter().map(|&x| (x, $T::from_order(x))).collect();

                let group = format_group_template!($T, $U, "vertices_iter");
                let mut group = c.benchmark_group(group);
                for (n, i) in &gs {
                    group.bench_with_input(BenchmarkId::from_parameter(n), &i, |b, &i| {
                        b.iter(|| {
                            black_box(i).vertices_iter().count();
                        })
                    });
                }
            }

            fn [< $T:lower _ $U:lower _ edges_iter >] (c: &mut Criterion) {
                let gs: Vec<(usize, Vec<($U, $U)>)> = N
                    .iter()
                    .map(|&x| (x, (0..x as $U).zip((0..x as $U).into_iter().rev()).collect()))
                    .collect();
                let gs: Vec<(usize, $T<$U>)> = gs.iter().map(|(x, y)| (*x, $T::from_edges(y))).collect();

                let group = format_group_template!($T, $U, "edges_iter");
                let mut group = c.benchmark_group(group);
                for (n, i) in &gs {
                    group.bench_with_input(BenchmarkId::from_parameter(n), &i, |b, &i| {
                        b.iter(|| {
                            black_box(i).edges_iter().count();
                        })
                    });
                }
            }

            criterion_group!(
                [< $T:lower _ $U:lower _ storage >],
                [< $T:lower _ $U:lower _ new >],
                [< $T:lower _ $U:lower _ with_capacity >],
                [< $T:lower _ $U:lower _ reserve >],
                [< $T:lower _ $U:lower _ from_order >],
                [< $T:lower _ $U:lower _ from_vertices >],
                [< $T:lower _ $U:lower _ from_edges >],
                [< $T:lower _ $U:lower _ vertices_iter >],
                [< $T:lower _ $U:lower _ edges_iter >]
            );
        }
    };
}

criterion_storage_group_template!(AdjacencyList, i32);

criterion_main!(adjacencylist_i32_storage);
