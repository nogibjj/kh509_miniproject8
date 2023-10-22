use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::time::Instant;
use std::mem;
use libc::{self, RUSAGE_SELF, timeval, rusage};

#[derive(Debug, Deserialize)]
pub struct IrisData {
    #[serde(rename = "sepal.length")]
    pub sepal_length: f64,
    #[serde(rename = "sepal.width")]
    pub sepal_width: f64,
    #[serde(rename = "petal.length")]
    pub petal_length: f64,
    #[serde(rename = "petal.width")]
    pub petal_width: f64,
    #[serde(rename = "variety")]
    pub class: String,
}
pub fn read_iris_data(file_path: &str) -> Result<Vec<IrisData>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut data = Vec::new();
    for record in reader.deserialize() {
        let record: IrisData = record?;
        data.push(record);
    }
    Ok(data)
}

fn calculate_summary_stats(data: &[IrisData]) -> (f64, f64, f64, f64) {
    let sepal_length: Vec<f64> = data.iter().map(|entry| entry.sepal_length).collect();
    let mean = sepal_length.iter().sum::<f64>() / sepal_length.len() as f64;

    let variance = sepal_length
        .iter()
        .map(|x| (*x - mean).powi(2))
        .sum::<f64>()
        / sepal_length.len() as f64;

    let std_dev = variance.sqrt();

    let min_value = *sepal_length.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap_or(&0.0);
    let max_value = *sepal_length.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap_or(&0.0);

    (mean, std_dev, min_value, max_value)
}
fn get_resource_usage() -> rusage {
    let mut usage: rusage = unsafe { mem::zeroed() };
    let _ = unsafe { libc::getrusage(RUSAGE_SELF, &mut usage) };
    usage
}

fn timeval_to_seconds(tv: timeval) -> f64 {
    tv.tv_sec as f64 + (tv.tv_usec as f64 / 1_000_000.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let usage_start = get_resource_usage();

    let data = read_iris_data("iris.csv")?;
    let (mean, std_dev, min_value, max_value) = calculate_summary_stats(&data);

    println!("Mean Sepal Length: {:.2}", mean);
    println!("Standard Deviation Sepal Length: {:.2}", std_dev);
    println!("Minimum Sepal Length: {:.2}", min_value);
    println!("Maximum Sepal Length: {:.2}", max_value);

    let end_time = Instant::now();
    let execution_time = end_time.duration_since(start_time);

    let usage_end = get_resource_usage();

    let user_cpu_time = timeval_to_seconds(usage_end.ru_utime) - timeval_to_seconds(usage_start.ru_utime);
    let system_cpu_time = timeval_to_seconds(usage_end.ru_stime) - timeval_to_seconds(usage_start.ru_stime);

    println!("User CPU time: {:.6} seconds", user_cpu_time);
    println!("System CPU time: {:.6} seconds", system_cpu_time);

    println!("Execution time: {:?}", execution_time);
    Ok(())
}

