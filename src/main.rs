mod graph;
mod models;
mod nn;

use models::{exponential_predictor, sine_predictor};

fn main() {
    run_exponential_predictor();
    println!("Fin!");
}

fn run_exponential_predictor() {
    let model = exponential_predictor::build_model();
    exponential_predictor::train_model(&model);
    exponential_predictor::test_model(&model);
}

fn run_sine_predictor() {
    let (input, mlp) = sine_predictor::build_model();
    sine_predictor::train_model(&input, &mlp, 10, 25, 1e-6);
    //sine_predictor::train_model(&input, &mlp, 1, 10, 1e-3);
    sine_predictor::test_model(&input, &mlp);
}
