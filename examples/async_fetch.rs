use open_macros::*;

script_async! {
    say!("openMacros async example");

    dir_mkp!("target/om-async-demo")?;
    file_write!("target/om-async-demo/input.txt", "from sync setup")?;

    sleep_async!(10);
    let body = file_read_async!("target/om-async-demo/input.txt").await?;
    say!("read async: {}", body);

    let echoed = cmd_async!("echo async_fetch_ok").await?;
    when!(echoed == "async_fetch_ok" => {
        say!("cmd_async ok");
    }, else => {
        bail!("unexpected cmd output");
    });
}
