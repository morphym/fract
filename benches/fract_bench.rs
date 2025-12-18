use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use fract::ChaosFiber256;

fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");

    for size in [64, 256, 1024, 4096, 16384, 65536].iter() {
        let data = vec![0x61u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(format!("hash_{}_bytes", size), &data, |b, data| {
            b.iter(|| ChaosFiber256::hash(black_box(data)));
        });
    }

    group.finish();
}

fn bench_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency");

    // Test small inputs (typical for many use cases)
    for size in [1, 4, 16, 32, 64, 128].iter() {
        let data = vec![0x61u8; *size];

        group.bench_with_input(format!("hash_{}_bytes_latency", size), &data, |b, data| {
            b.iter(|| ChaosFiber256::hash(black_box(data)));
        });
    }

    group.finish();
}

fn bench_incremental(c: &mut Criterion) {
    let mut group = c.benchmark_group("incremental");

    let chunk_sizes = [64, 256, 1024];
    let total_size = 4096;

    for chunk_size in &chunk_sizes {
        let data = vec![0x61u8; total_size];
        let num_chunks = total_size / chunk_size;

        group.throughput(Throughput::Bytes(total_size as u64));
        group.bench_function(
            format!("incremental_{}_byte_chunks", chunk_size),
            |b| {
                b.iter(|| {
                    let mut hasher = ChaosFiber256::new();
                    for chunk in data.chunks(*chunk_size) {
                        hasher.update(black_box(chunk));
                    }
                    black_box(hasher.finalize())
                });
            },
        );
    }

    group.finish();
}

fn bench_512bit(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash512");

    for size in [64, 256, 1024, 4096].iter() {
        let data = vec![0x61u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(format!("hash512_{}_bytes", size), &data, |b, data| {
            b.iter(|| ChaosFiber256::hash512(black_box(data)));
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_throughput,
    bench_latency,
    bench_incremental,
    bench_512bit
);
criterion_main!(benches);
