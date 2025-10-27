use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use finders::file_finder::Finder;
use finders::search_files;
use finders::searcher::{ReSearcher, Searcher, Searches};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn setup_test_files(dir: &PathBuf, num_files: usize, lines_per_file: usize) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for i in 0..num_files {
        let file_path = dir.join(format!("test_file_{}.txt", i));
        let mut file = File::create(&file_path).unwrap();

        for j in 0..lines_per_file {
            writeln!(
                file,
                "This is line {} in file {} with some searchable content",
                j, i
            )
            .unwrap();
        }

        paths.push(file_path);
    }

    paths
}

fn cleanup_test_files(paths: &[PathBuf]) {
    for path in paths {
        let _ = fs::remove_file(path);
    }
}

fn bench_file_finder(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_finder");

    // Benchmark finding files in current directory
    group.bench_function("find_all_files", |b| {
        b.iter(|| {
            let finder = Finder::new(Some(".")).unwrap();
            black_box(finder.find(None))
        });
    });

    // Benchmark finding files with pattern
    group.bench_function("find_rust_files", |b| {
        b.iter(|| {
            let finder = Finder::new(Some(".")).unwrap();
            black_box(finder.find(Some(".rs")))
        });
    });

    group.finish();
}

fn bench_searching(c: &mut Criterion) {
    let mut group = c.benchmark_group("searching");

    // Test content with varying sizes
    let contents_small = "line one\nLINE TWO\nline three\nLINE FOUR";
    let contents_medium: String = (0..100)
        .map(|i| format!("This is line {} with some content\n", i))
        .collect();
    let contents_large: String = (0..1000)
        .map(|i| format!("This is line {} with some content\n", i))
        .collect();

    // Benchmark case-sensitive search on small content
    group.bench_function("search_case_sensitive_small", |b| {
        let searcher = Searcher::new("line", false);
        b.iter(|| black_box(searcher.search(&contents_small)));
    });

    // Benchmark case-insensitive search on small content
    group.bench_function("search_case_insensitive_small", |b| {
        let searcher = Searcher::new("line", true);
        b.iter(|| black_box(searcher.search(&contents_small)));
    });

    // Benchmark regex search on small content
    group.bench_function("search_regex_small", |b| {
        let re_searcher = ReSearcher::new("[a-z]+");
        b.iter(|| black_box(re_searcher.search(&contents_small)));
    });

    // Benchmark case-sensitive search on medium content
    group.bench_function("search_case_sensitive_medium", |b| {
        let searcher = Searcher::new("line", false);
        b.iter(|| black_box(searcher.search(&contents_medium)));
    });

    // Benchmark case-sensitive search on large content
    group.bench_function("search_case_sensitive_large", |b| {
        let searcher = Searcher::new("line", false);
        b.iter(|| black_box(searcher.search(&contents_large)));
    });

    group.finish();
}

fn bench_streaming_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming_search");

    let temp_dir = std::env::temp_dir().join("finders_bench");
    fs::create_dir_all(&temp_dir).unwrap();

    // Benchmark with different file counts and sizes
    for &(num_files, lines_per_file) in &[(1, 100), (5, 100), (1, 1000), (10, 100)] {
        let paths = setup_test_files(&temp_dir, num_files, lines_per_file);

        group.bench_with_input(
            BenchmarkId::new(
                "stream_search",
                format!("{}files_{}lines", num_files, lines_per_file),
            ),
            &paths,
            |b, paths| {
                b.iter(|| {
                    // Create searcher for each iteration since it's consumed
                    let searcher = Searcher::new("searchable", false);
                    // Redirect stdout to avoid benchmark overhead from printing
                    let _result = search_files(searcher, paths.clone(), false);
                });
            },
        );

        cleanup_test_files(&paths);
    }

    // Clean up the temporary directory
    let _ = fs::remove_dir(&temp_dir);

    group.finish();
}

fn bench_search_line(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_line");

    let test_line = "This is a test line with searchable content";

    // Benchmark search_line for case-sensitive search
    group.bench_function("search_line_case_sensitive", |b| {
        let searcher = Searcher::new("searchable", false);
        b.iter(|| black_box(searcher.search_line(test_line, 1)));
    });

    // Benchmark search_line for case-insensitive search
    group.bench_function("search_line_case_insensitive", |b| {
        let searcher = Searcher::new("SEARCHABLE", true);
        b.iter(|| black_box(searcher.search_line(test_line, 1)));
    });

    // Benchmark search_line for regex search
    group.bench_function("search_line_regex", |b| {
        let re_searcher = ReSearcher::new("search[a-z]+");
        b.iter(|| black_box(re_searcher.search_line(test_line, 1)));
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_file_finder,
    bench_searching,
    bench_streaming_search,
    bench_search_line
);
criterion_main!(benches);
