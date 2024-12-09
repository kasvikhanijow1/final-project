use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

//this code reads the CSV file and extracts data for days (x) and total streams (y).
pub fn read_csv(file_path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut days = Vec::new();
    let mut total_streams = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue; 
        }
        let cols: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if cols.len() < 9 {
            days.push(0.0);
            total_streams.push(0.0);
            continue;
        }

        let days_value = cols[3]; 
        let streams_value = cols[8]; 

        let parsed_days = days_value.parse::<f64>().unwrap_or(0.0);
        let parsed_streams = streams_value.parse::<f64>().unwrap_or(0.0);

        days.push(parsed_days);
        total_streams.push(parsed_streams);
    }

    Ok((days, total_streams))
}

