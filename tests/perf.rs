extern crate astral_futures_tar as async_tar;

use std::{
    fs,
    future::Future,
    path::Path,
    time::{Duration, Instant},
};

#[cfg(feature = "futures")]
use futures_lite::io::{AsyncReadExt, Cursor};
use futures_lite::StreamExt;
#[cfg(feature = "tokio")]
use std::io::Cursor;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;

use async_tar::{Archive, Builder, Header};
use tempfile::TempDir;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => panic!("{} returned {}", stringify!($e), e),
        }
    };
}

#[cfg(feature = "futures")]
fn test_async<F: Future<Output = ()>>(future: F) {
    futures_lite::future::block_on(future);
}

#[cfg(feature = "tokio")]
fn test_async<F: Future<Output = ()>>(future: F) {
    tokio::runtime::Runtime::new().unwrap().block_on(future);
}

const ENTRY_COUNT: usize = 128;
const ENTRY_SIZE: usize = 32 * 1024;
const LARGE_ENTRY_COUNT: usize = 32;
const LARGE_ENTRY_SIZE: usize = 1024 * 1024;
const FS_DIRS: usize = 16;
const FS_FILES_PER_DIR: usize = 8;
const FS_FILE_SIZE: usize = 16 * 1024;
const RUNS: usize = 5;

#[ignore]
#[test]
fn perf_streaming_write() {
    test_async(async {
        let payload = payload(ENTRY_SIZE);

        let best = repeat(RUNS, || async {
            let mut builder = Builder::new(Vec::new());
            for index in 0..ENTRY_COUNT {
                let mut header = Header::new_gnu();
                t!(header.set_path(format!("file-{index:04}.bin")));
                header.set_size(payload.len() as u64);
                header.set_cksum();
                t!(builder.append(&header, &payload[..]).await);
            }
            let archive = t!(builder.into_inner().await);
            assert!(archive.len() >= ENTRY_COUNT * ENTRY_SIZE);
        })
        .await;

        report("streaming_write", best, ENTRY_COUNT * ENTRY_SIZE);
    });
}

#[ignore]
#[test]
fn perf_streaming_read() {
    test_async(async {
        let archive = streaming_archive().await;

        let best = repeat(RUNS, || {
            let archive = archive.clone();
            async move {
                let mut archive = Archive::new(Cursor::new(archive));
                let mut entries = t!(archive.entries());
                let mut count = 0;
                while let Some(entry) = entries.next().await {
                    let mut entry = t!(entry);
                    let mut sink = Vec::new();
                    t!(entry.read_to_end(&mut sink).await);
                    count += 1;
                }
                assert_eq!(count, ENTRY_COUNT);
            }
        })
        .await;

        report("streaming_read", best, ENTRY_COUNT * ENTRY_SIZE);
    });
}

#[ignore]
#[test]
fn perf_streaming_read_large() {
    test_async(async {
        let archive = streaming_archive_with(LARGE_ENTRY_COUNT, LARGE_ENTRY_SIZE).await;

        let best = repeat(RUNS, || {
            let archive = archive.clone();
            async move {
                let mut archive = Archive::new(Cursor::new(archive));
                let mut entries = t!(archive.entries());
                let mut count = 0;
                while let Some(entry) = entries.next().await {
                    let mut entry = t!(entry);
                    let mut sink = Vec::new();
                    t!(entry.read_to_end(&mut sink).await);
                    count += 1;
                }
                assert_eq!(count, LARGE_ENTRY_COUNT);
            }
        })
        .await;

        report(
            "streaming_read_large",
            best,
            LARGE_ENTRY_COUNT * LARGE_ENTRY_SIZE,
        );
    });
}

#[ignore]
#[test]
fn perf_filesystem_pack() {
    test_async(async {
        let fixture = fs_fixture();
        let bytes = FS_DIRS * FS_FILES_PER_DIR * FS_FILE_SIZE;

        let best = repeat(RUNS, || {
            let root = fixture.path().to_path_buf();
            async move {
                let mut builder = Builder::new(Vec::new());
                t!(builder.append_dir_all("fixture", &root).await);
                let archive = t!(builder.into_inner().await);
                assert!(archive.len() >= bytes);
            }
        })
        .await;

        report("filesystem_pack", best, bytes);
    });
}

#[ignore]
#[test]
fn perf_filesystem_unpack() {
    test_async(async {
        let fixture = fs_fixture();
        let mut builder = Builder::new(Vec::new());
        t!(builder.append_dir_all("fixture", fixture.path()).await);
        let archive = t!(builder.into_inner().await);
        let bytes = FS_DIRS * FS_FILES_PER_DIR * FS_FILE_SIZE;

        let best = repeat(RUNS, || {
            let archive = archive.clone();
            async move {
                let dst = t!(tempfile::Builder::new().prefix("tar-perf-unpack").tempdir());
                let mut archive = Archive::new(Cursor::new(archive));
                t!(archive.unpack(dst.path()).await);
                assert!(dst.path().join("fixture").exists());
            }
        })
        .await;

        report("filesystem_unpack", best, bytes);
    });
}

async fn streaming_archive() -> Vec<u8> {
    streaming_archive_with(ENTRY_COUNT, ENTRY_SIZE).await
}

async fn streaming_archive_with(entry_count: usize, entry_size: usize) -> Vec<u8> {
    let payload = payload(entry_size);
    let mut builder = Builder::new(Vec::new());
    for index in 0..entry_count {
        let mut header = Header::new_gnu();
        t!(header.set_path(format!("file-{index:04}.bin")));
        header.set_size(payload.len() as u64);
        header.set_cksum();
        t!(builder.append(&header, &payload[..]).await);
    }
    t!(builder.into_inner().await)
}

fn fs_fixture() -> TempDir {
    let dir = t!(tempfile::Builder::new()
        .prefix("tar-perf-fixture")
        .tempdir());
    for dir_index in 0..FS_DIRS {
        let subdir = dir.path().join(format!("dir-{dir_index:03}"));
        t!(fs::create_dir(&subdir));
        for file_index in 0..FS_FILES_PER_DIR {
            write_payload(
                &subdir.join(format!("file-{file_index:03}.bin")),
                FS_FILE_SIZE,
                dir_index * FS_FILES_PER_DIR + file_index,
            );
        }
    }
    dir
}

fn payload(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i % 251) as u8).collect()
}

fn write_payload(path: &Path, len: usize, seed: usize) {
    let data: Vec<u8> = (0..len).map(|i| ((i + seed) % 251) as u8).collect();
    t!(fs::write(path, data));
}

async fn repeat<F, Fut>(runs: usize, mut f: F) -> Duration
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    f().await;

    let mut best = Duration::MAX;
    for _ in 0..runs {
        let started = Instant::now();
        f().await;
        best = best.min(started.elapsed());
    }
    best
}

fn report(name: &str, elapsed: Duration, bytes: usize) {
    let mib = bytes as f64 / 1024.0 / 1024.0;
    let seconds = elapsed.as_secs_f64();
    eprintln!(
        "{} {name}: best_of_{RUNS}={elapsed:?}, bytes={bytes}, throughput_mib_s={:.2}",
        backend_name(),
        mib / seconds
    );
}

#[cfg(feature = "futures")]
fn backend_name() -> &'static str {
    "futures"
}

#[cfg(feature = "tokio")]
fn backend_name() -> &'static str {
    "tokio"
}
