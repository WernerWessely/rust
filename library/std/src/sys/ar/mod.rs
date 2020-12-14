//! System bindings for the AR.
//! There is no way to detect use of unsupported features are compile time yet.
//! Keep an eye on the portability lint: https://github.com/rust-lang/rust/issues/41619

#[path = "../unsupported/alloc.rs"]
pub mod alloc;
#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../unsupported/cmath.rs"]
pub mod cmath;
#[path = "../unsupported/common.rs"]
mod common;
#[path = "../unsupported/condvar.rs"]
pub mod condvar;
#[path = "../unsupported/env.rs"]
pub mod env;
#[path = "../unsupported/fs.rs"]
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
#[path = "../unsupported/mutex.rs"]
pub mod mutex;
#[path = "../unsupported/net.rs"]
pub mod net;
#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
#[path = "../unsupported/rwlock.rs"]
pub mod rwlock;
#[path = "../unsupported/stack_overflow.rs"]
pub mod stack_overflow;
#[path = "../unsupported/stdio.rs"]
pub mod stdio;
#[path = "../unsupported/thread.rs"]
pub mod thread;
#[path = "../unsupported/thread_local_dtor.rs"]
pub mod thread_local_dtor;
#[path = "../unsupported/thread_local_key.rs"]
pub mod thread_local_key;
#[path = "../unsupported/time.rs"]
pub mod time;
pub use common::*;
