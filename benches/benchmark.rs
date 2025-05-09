use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput, black_box};

use magnet_url::{Magnet, MagnetBuilder};

fn benchmark_magnet_parsing(c: &mut Criterion) {
    let magnet = black_box("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
    let mut group = c.benchmark_group("magnet_parsing");

    group.throughput(Throughput::Elements(1));

    // Benchmark parsing
    group.bench_with_input(BenchmarkId::new("parse", magnet), &magnet, |b, &s| {
        b.iter(|| Magnet::new(s));
    });

    group.finish();
}

fn benchmark_to_string(c: &mut Criterion) {
    let magnet = black_box(
        Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent").unwrap()
    );
    
    let mut group = c.benchmark_group("to_string");
    
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("to_string", |b| {
        b.iter(|| black_box(&magnet).to_string());
    });
    
    group.finish();
}

fn benchmark_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("builder");
    
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("create", |b| {
        b.iter(|| {
            MagnetBuilder::new()
                .display_name("Test")
                .hash_type("btih")
                .hash("1234567890abcdef1234567890abcdef12345678")
                .length(12345)
                .add_tracker("udp://tracker1.example.com:6969")
                .add_tracker("udp://tracker2.example.com:6969")
                .search_keywords("test+keywords")
                .web_seed("https://example.com/seed")
                .acceptable_source("https://example.com/download")
                .manifest("https://example.com/manifest")
                .source("https://example.com/source")
                .build()
        });
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_magnet_parsing, benchmark_to_string, benchmark_builder);
criterion_main!(benches);