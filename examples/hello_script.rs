use open_macros::*;

script! {
    say!("Hello from openMacros 2.0");

    let dir = path_join!("target", "om-demo");
    dir_mkp!(&dir)?;
    file_write!(dir.join("note.txt"), "short rust scripts")?;

    let text = file_read!(dir.join("note.txt"))?;
    say!("read back: {}", trim!(&text));

    unless!(text.is_empty() => {
        say!("file has content");
    });
}
