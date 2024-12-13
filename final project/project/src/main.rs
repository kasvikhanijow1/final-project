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
}

mod tests {
    use super::*;
    #[test]
    fn test_normalize() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let (normalized, mean, std) = normalize(&data);
        assert_eq!(mean, 3.0, "Mean should be 3.0.");
        assert!((std - (2.0f64).sqrt()).abs() < 1e-6, "Std should be sqrt(2).");
        assert_eq!(normalized.len(), data.len(), "Normalized data length should match original.");
    }

    #[test]
    fn test_linear_regression() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        let y = vec![2.0, 4.0, 6.0, 8.0]; 
        let (slope, intercept) = linear_regression(&x, &y, 0.01, 10_000);
        assert!((slope - 2.0).abs() < 0.001, "Slope should be close to 2.0.");
        assert!(intercept.abs() < 0.001, "Intercept should be close to 0.0.");
    }

    #[test]
    fn test_coefficient_of_determination() {
        let x = vec![1.0, 2.0, 3.0, 4.0];
        let y = vec![2.0, 4.0, 6.0, 8.0];
        let (slope, intercept) = linear_regression(&x, &y, 0.01, 10_000);
        let r2 = coefficient_of_determination(&x, &y, slope, intercept);
        assert!((r2 - 1.0).abs() < 0.001, "R^2 should be close to 1.0 for a perfect fit.");
    }
}

