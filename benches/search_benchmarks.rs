use criterion::{Criterion, black_box, criterion_group, criterion_main};
use finders::searcher::Searches;
use finders::{file_finder, searcher};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn create_test_files(dir: &PathBuf, num_files: usize, lines_per_file: usize) {
    fs::create_dir_all(dir).unwrap();
    for i in 0..num_files {
        let file_path = dir.join(format!("test_file_{}.txt", i));
        let mut file = fs::File::create(&file_path).unwrap();
        for j in 0..lines_per_file {
            writeln!(
                file,
                "This is line {} in file {} with some searchable content",
                j, i
            )
            .unwrap();
        }
    }
}

fn cleanup_test_files(dir: &PathBuf) {
    let _ = fs::remove_dir_all(dir);
}

fn bench_searcher_search_line(c: &mut Criterion) {
    let searcher_case_sensitive = searcher::Searcher::new("searchable", false);
    let searcher_case_insensitive = searcher::Searcher::new("SEARCHABLE", true);
    let line = "This is a line with some searchable content in it";

    let mut group = c.benchmark_group("searcher_search_line");

    group.bench_function("case_sensitive", |b| {
        b.iter(|| searcher_case_sensitive.search_line(black_box(line), black_box(1)))
    });

    group.bench_function("case_insensitive", |b| {
        b.iter(|| searcher_case_insensitive.search_line(black_box(line), black_box(1)))
    });

    group.finish();
}

fn bench_regex_searcher_search_line(c: &mut Criterion) {
    let regex_searcher = searcher::ReSearcher::new(r"search\w+");
    let line = "This is a line with some searchable content in it";

    c.bench_function("regex_searcher_search_line", |b| {
        b.iter(|| regex_searcher.search_line(black_box(line), black_box(1)))
    });
}

fn bench_searcher_search_content(c: &mut Criterion) {
    let searcher = searcher::Searcher::new("searchable", false);
    let content = "This is line 1 with some searchable content\n\
                   This is line 2 with more content\n\
                   This is line 3 with searchable data\n";

    let mut group = c.benchmark_group("searcher_search_content");

    group.bench_function("search_multiline", |b| {
        b.iter(|| searcher.search(black_box(content)))
    });

    group.finish();
}

fn bench_file_finder(c: &mut Criterion) {
    let temp_dir = std::env::temp_dir().join("finders_bench_finder");
    create_test_files(&temp_dir, 100, 10);

    let temp_dir_str = temp_dir.to_str().unwrap();

    let mut group = c.benchmark_group("file_finder");

    group.bench_function("find_all_files", |b| {
        b.iter(|| {
            let finder = file_finder::Finder::new(Some(black_box(temp_dir_str))).unwrap();
            finder.find(None)
        })
    });

    group.bench_function("find_with_pattern", |b| {
        b.iter(|| {
            let finder = file_finder::Finder::new(Some(black_box(temp_dir_str))).unwrap();
            finder.find(Some(black_box("test_file_5")))
        })
    });

    group.finish();
    cleanup_test_files(&temp_dir);
}

criterion_group!(
    benches,
    bench_searcher_search_line,
    bench_regex_searcher_search_line,
    bench_searcher_search_content,
    bench_file_finder
);
criterion_main!(benches);
