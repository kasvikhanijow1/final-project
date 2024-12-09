mod modules;

use modules::read_csv;
use modules::normalize;
use modules::linear_regression;
use modules::coefficient_of_determination;


//i used the main function for it to reads the data, normalizes it, performs linear regression, evaluates the model with R², 
//and prints the results according to the data set
fn main() {
    let file_path = "Spotify_final_dataset.csv";

    let (days, total_streams) = match read_csv(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading CSV: {}", e);
            return;
        }
    };

    if days.is_empty() || total_streams.is_empty() {
        eprintln!("No valid data to process.");
        return;
    }

    let (norm_days, _, _) = normalize(&days);
    let (norm_streams, _, _) = normalize(&total_streams);

    let learning_rate = 0.01;
    let iterations = 10_000;
    let (slope, intercept) =
        linear_regression(&norm_days, &norm_streams, learning_rate, iterations);

    let r2 = coefficient_of_determination(&norm_days, &norm_streams, slope, intercept);

    println!("Linear Regression Results:");
    println!("Slope (m): {:.4}", slope);
    println!("Intercept (b): {:.4}", intercept);
    println!("Equation: y = {:.4}x + {:.4}", slope, intercept);
    println!("Coefficient of Determination (R^2): {:.4}", r2);

