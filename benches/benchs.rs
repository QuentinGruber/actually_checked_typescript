use std::path::PathBuf;

use act_lib::patch_index_helper::PatchIndexHelper;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_patch_index_helper(c: &mut Criterion) {
    c.bench_function("patch_index_helper_register_indexes", |b| {
        let mut index = PatchIndexHelper::new();
        b.iter(|| {
            index.register_patched_index(0, 1);
        })
    });
    c.bench_function("patch_index_helper_get_drifted_index_on_100_indexes", |b| {
        let mut index = PatchIndexHelper::new();
        for i in 0..100 {
            index.register_patched_index(i, 10);
        }
        b.iter(|| {
            index.get_drifted_index(black_box(0));
        })
    });
}

fn bench_process_file(c: &mut Criterion) {
    c.bench_function("process_file", |b| {
        let file_path = PathBuf::from("ts_example/test.ts");
        b.iter(|| {
            act_lib::act_process::process_file(black_box(file_path.clone())).unwrap_or(());
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    // bench_patch_index_helper(c);
    bench_process_file(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
