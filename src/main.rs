use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_iris_data("iris.csv")?;
    let (mean, std_dev, min_value, max_value) = calculate_summary_stats(&data);

    println!("Mean Sepal Length: {:.2}", mean);
    println!("Standard Deviation Sepal Length: {:.2}", std_dev);
    println!("Minimum Sepal Length: {:.2}", min_value);
    println!("Maximum Sepal Length: {:.2}", max_value);

    Ok(())
}
