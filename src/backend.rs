pub(crate) use std::io;

#[cfg(all(feature = "tokio", not(feature = "futures")))]
pub(crate) use tokio::{
    fs::{self, File, OpenOptions},
    io::{
        copy, empty, repeat, AsyncRead as Read, AsyncReadExt, AsyncSeekExt, AsyncWrite as Write,
        AsyncWriteExt, BufWriter, ReadBuf, Repeat, Take,
    },
    sync::Mutex,
};

#[cfg(all(feature = "tokio", not(feature = "futures")))]
pub(crate) use tokio_stream::StreamExt;

#[cfg(all(feature = "futures-fs", not(feature = "tokio")))]
pub(crate) use async_fs::{self as fs, File, OpenOptions};

#[cfg(all(feature = "futures", not(feature = "tokio")))]
pub(crate) use async_lock::Mutex;

#[cfg(all(feature = "futures", not(feature = "tokio")))]
pub(crate) use futures_lite::{
    io::{
        copy, repeat, AsyncRead as Read, AsyncReadExt, AsyncSeekExt, AsyncWrite as Write,
        AsyncWriteExt, BufWriter, Repeat, Take,
    },
    StreamExt,
};

#[cfg(all(feature = "futures-fs", not(feature = "tokio")))]
pub(crate) use futures_lite::io::empty;
