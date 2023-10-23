# Mini-Project 8: Rewrite a Python Script in Rust
Katelyn Hucker (kh509)

[![Clippy](https://github.com/nogibjj/kh509_miniproject8/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/kh509_miniproject8/actions/workflows/cicd.yml)


# Objective:
The objective of this project is to rewrite an existing Python script for data processing in Rust, highlighting improvements in speed and resource usage. For the same, I re-wrote my [Week 3 Python Polars Mini Project]((https://github.com/nogibjj/kh509_miniproject3)) in Rust. This was a simple project with the following functions:
* Calculate Mean
* Calculate Median
* Calculate Standard Deviation

# How is this project different:
_______________________________

## Project Structure:
```
kh509_miniproject8/
├── .github/
│   └── workflows/cicd.yml
├── .devcontainer/
    ├── Dockerfile
│   └── devcontainer.json
├── .gitignore
├── src/
│   └── main.rs
├── Cargo.toml
├── Makefile
└── README.md
```

### main.rs:
main.rs is a statistics script that takes advantage of the polars library in *Rust*. The script imports a csv file as a dataframe, of the developers choosing, in this case it is the iris.csv dataset. The dataset contains varying numeric information about iris flowers. The whole dataset is imported with the read_csv method. The script contains a function that imports a dataframe's column, calculates and returns the descriptive stats of the column. It outputs the mean, standard deviation, min, and max of the petal length column. 


# Running the project
_______________________________
![image](https://github.com/nogibjj/kh509_miniproject8/assets/143521756/34bdf964-4a8c-441f-8713-7dc2689029ef)


# Python vs Rust:
_______________________________
| Language | Execution Time | User CPU time | System CPU time |
|----------|----------------|---------------|----------------|
| Python   | 1.091057 seconds | 1.021171s     | 2.026013s      |
| Rust     | 1.060837ms    | 0.000508s     | 0.000508s      |


We see here that there is a major difference just in these small math based scripts. I can only imagine how much longer a whole computationally heavy project would be in python rather than in Rust. 
