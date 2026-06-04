#![cfg(feature = "async")]

use open_macros::*;

#[tokio::test]
async fn async_file_and_sleep_work() {
    let path = "target/om-async-test.txt";
    let _ = std::fs::remove_file(path);
    file_write_async!(path, "async data")
        .await
        .expect("async write");
    sleep_async!(1);
    let content = file_read_async!(path).await.expect("async read");
    assert_eq!(content, "async data");
    let _ = std::fs::remove_file(path);
}

#[tokio::test]
async fn cmd_async_runs_successfully() {
    let output = cmd_async!("echo async_ok").await.expect("async command");
    assert_eq!(output, "async_ok");
}
