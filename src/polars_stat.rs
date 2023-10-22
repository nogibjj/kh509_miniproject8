use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read CSV file into a DataFrame
    let df1 = CsvReader::from_path("iris.csv")?.finish()?;

    // Calculate descriptive statistics for a specific column
    let petal_length_stats = calc_desc_stat(&df1["petal.length"])?;

    println!("{:?}", petal_length_stats);

    Ok(())
}

fn calc_desc_stat(dataset_col: &Series) -> Result<DataFrame> {
    // Calculate descriptive statistics for the given column (Series)
    let out = dataset_col.describe()?;

    Ok(out)
}
