extern crate astral_futures_tar as async_tar;

use async_tar::Archive;
use futures_lite::{future::block_on, StreamExt};

fn main() {
    block_on(async {
        let file = async_fs::File::open("archive.tar").await.unwrap();
        let mut archive = Archive::new(file);
        let mut entries = archive.entries().unwrap();

        while let Some(file) = entries.next().await {
            let file = file.unwrap();
            println!("{}", file.path().unwrap().display());
        }
    })
}
