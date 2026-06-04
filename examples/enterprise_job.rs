use open_macros::*;

script! {
    say!("Starting enterprise automation job");

    repeat!(step in 1 => 4, {
        say!("running step {}", step);
    });

    let output = cmd!("echo pipeline_ok")?;
    when!(output == "pipeline_ok" => {
        say!("pipeline health: {}", output);
    }, else => {
        bail!("pipeline health check failed");
    });

    file_write!("job_report.txt", format!("result={output}\n"))?;
}
