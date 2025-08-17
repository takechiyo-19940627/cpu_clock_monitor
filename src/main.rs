use std::process::Command;
use regex::Regex;
use tokio::time::{sleep, Duration};


struct CpuFrequency {
    p_cluster: f64,
    e_cluster: f64,
    timestamp: std::time::SystemTime,
}

async fn get_cpu_frequency() -> Result<CpuFrequency, Box<dyn std::error::Error>> {
    let output = Command::new("sudo")
        .args(&["powermetrics", "-i", "1000", "-n", "1", "--samplers", "cpu_power", "-f", "json"])
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;

    let p_regex = Regex::new(r"P-Cluster HW active frequency: (\d+\.?\d*) MHz")?;
    let e_regex = Regex::new(r"E-Cluster HW active frequency: (\d+\.?\d*) MHz")?;

    let p_freq = p_regex.captures(&stdout)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .unwrap_or(0.0);

    let e_freq = e_regex.captures(&stdout)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .unwrap_or(0.0);

    Ok(CpuFrequency {
        p_cluster: p_freq,
        e_cluster: e_freq,
        timestamp: std::time::SystemTime::now(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CPU Clock Monitor - Press Ctrl+C to stop");
    println!("{:<20} {:<15} {:<15}", "Time", "P-Core(MHz)", "E-Core(MHz)");
    println!("{}", "-".repeat(50));

    loop {
        match get_cpu_frequency().await {
            Ok(freq) => {
                let time = freq.timestamp
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs();
                println!("{:<20} {:<15} {:<15}", time, freq.p_cluster, freq.e_cluster);
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        sleep(Duration::from_secs(2)).await;
    }
}
