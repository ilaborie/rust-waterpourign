#[macro_use]
extern crate criterion;

use criterion::{BenchmarkId, Criterion};

use waterpouring_imp::imp::ImperativeSolver;
use waterpouring_model::problem::Problem;
use waterpouring_model::solver::Solver;
use waterpouring_rec::rec::RecSolver;
use waterpouring_rec2::rec2::Rec2Solver;

pub fn criterion_benchmark(crit: &mut Criterion) {
    let problems: Vec<Problem> = vec![
        Problem::from(("0/5, 0/3", "4/5, 0/3")),
        Problem::from(("12/12, 0/8, 0/5", "6/12, 6/8, 0/5")),
    ];

    let mut group = crit.benchmark_group("waterpouring");
    for problem in problems {
        group.bench_with_input(
            BenchmarkId::new("rec", problem.clone()),
            &(problem.clone()),
            |b, s| {
                b.iter(|| {
                    let solver = RecSolver();
                    solver.solve(s.clone()).expect("Should work");
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("rec2", problem.clone()),
            &(problem.clone()),
            |b, s| {
                b.iter(|| {
                    let solver = Rec2Solver();
                    solver.solve(s.clone()).expect("Should work");
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("imp", problem.clone()),
            &(problem.clone()),
            |b, s| {
                b.iter(|| {
                    let solver = ImperativeSolver();
                    solver.solve(s.clone()).expect("Should work");
                })
            },
        );
    }
    group.finish()
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
