use ak_macros::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    say!("Starting enterprise automation job");

    repeat!(step in 1 => 4, {
        say!("running step {}", step);
    });

    let output = cmd!("echo pipeline_ok")?;
    when!(output == "pipeline_ok" => {
        say!("pipeline health: {}", output);
    }, else => {
        say!("pipeline health check failed");
    });

    file_write!("job_report.txt", format!("result={output}\n"))?;
    Ok(())
}
