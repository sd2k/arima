use arima::estimate::{autofit, fit};
use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

const CAF_EXPORTS: &[f64] = &[
    23.3, 26.5, 24.6, 25.2, 28.4, 27.1, 28.4, 26.3, 34.3, 27.3, 31.9, 29.8, 24.9, 28.6, 27.7, 21.1,
    22.2, 25.2, 23.4, 22.4, 25.2, 24.4, 22.2, 24.1, 23.4, 22.0, 18.2, 17.8, 17.7, 20.3, 17.1, 17.6,
    16.9, 17.1, 23.4, 22.2, 21.5, 26.9, 22.7, 19.2, 20.4, 17.2, 16.0, 18.2, 14.0, 13.4, 14.3, 14.1,
    11.0, 10.7, 11.8, 11.5, 11.6, 14.5, 13.0, 12.6, 12.7, 12.5,
];

fn fit_caf_exports(c: &mut Criterion) {
    c.bench_function("fit_caf_exports (2,1,0)", |b| {
        b.iter(|| fit(CAF_EXPORTS, 2, 1, 0));
    });
    c.bench_function("fit_caf_exports (0,1,3)", |b| {
        b.iter(|| fit(CAF_EXPORTS, 0, 1, 3));
    });
}

fn autofit_caf_exports(c: &mut Criterion) {
    c.bench_function("autofit_caf_exports", |b| {
        b.iter(|| autofit(CAF_EXPORTS, 1));
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Protobuf));
    targets = fit_caf_exports, autofit_caf_exports
}
criterion_main!(benches);
