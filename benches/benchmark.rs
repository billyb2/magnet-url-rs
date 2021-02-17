use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput, black_box};

extern crate magnet_url;
use magnet_url::Magnet;

fn criterion_benchmark(c: &mut Criterion) {
    let magnet = black_box("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
    let mut group = c.benchmark_group("sintel");

    group.throughput(Throughput::Elements(1));

    group.bench_with_input(BenchmarkId::new("sintel", magnet), &magnet, |b, &s| {
        b.iter(|| Magnet::new(s));
    });

    group.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);