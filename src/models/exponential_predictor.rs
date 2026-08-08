use crate::{
    graph::{ExpOp, MseOp, NodeRef, TanhOp},
    nn::Mlp,
};
use rand::prelude::*;

pub struct ExpPredictorModel {
    pub input: NodeRef,
    pub output: NodeRef,
    pub param: NodeRef,
}

pub fn build_model() -> ExpPredictorModel {
    let input = NodeRef::from(0.0);
    input.set_label("input");

    let exponential = NodeRef::from(1.0);
    exponential.set_label("exponential");

    let output = NodeRef::chained(&[input.clone(), exponential.clone()], &ExpOp);

    ExpPredictorModel { input, output, param: exponential }
}

pub fn train_model(model: &ExpPredictorModel) {
    let epochs = 4;
    let step = 1e-6;
    let train_dataset = build_dataset(500);

    let batch_size_node = NodeRef::from(1.0);
    let expected_node = NodeRef::from(0.0);
    let loss = NodeRef::chained(
        &[
            batch_size_node.clone(),
            expected_node.clone(),
            model.output.clone(),
        ],
        &MseOp,
    );

    println!();
    println!("Starting training for model ExponentialPredictor...");
    println!("Params: epochs={} step={}", epochs, step);
    println!();

    for epoch in 0..epochs {
        println!("Training Epoch {}", epoch);

        let mut avg_loss_sum = 0.0;

        for (input_value, expected) in &train_dataset {
            model.input.set_value(*input_value);
            expected_node.set_value(*expected);
            loss.clear_gradients();

            loss.compute_value();
            loss.compute_gradients();

            let param_value = model.param.value();
            let param_gradient = model.param.gradient();
            model.param.set_value(param_value - (param_gradient * step));

            avg_loss_sum += loss.value();
        }

        println!("  average loss={}", avg_loss_sum / train_dataset.len() as f32);
        println!("  param value={}", model.param.value());
    }

    println!("Training complete!");
    println!();
}

pub fn test_model(model: &ExpPredictorModel) {
    let dataset = build_dataset(25);

    println!("Starting testing...");
    println!("  dataset_length={}", dataset.len());

    for (input_value, expected_value) in dataset {
        model.input.set_value(input_value);

        model.output.compute_value();
        let actual_value = model.output.value();

        println!(
            "  input={} expected={} actual={} error={}",
            input_value,
            expected_value,
            actual_value,
            actual_value - expected_value
        );
    }

    println!();
}

fn build_dataset(length: usize) -> Vec<(f32, f32)> {
    (0..length)
        .map(|_| {
            let x = rand::rng().random::<f32>();
            let y = x.powf(2.0);
            (x, y)
        })
        .collect::<Vec<_>>()
}

