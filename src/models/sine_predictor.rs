use crate::engine::{MseOp, NodeRef, TanhOp, nn::Mlp};
use rand::prelude::*;

pub fn build_model() -> (NodeRef, Mlp) {
    let input = NodeRef::from(0.0);
    input.set_label("input");

    let mlp = Mlp::new(
        input.clone(),
        &[
            //(4, Some(&TanhOp)),
            (8, Some(&TanhOp)),
            //(2, Some(&TanhOp)),
            (1, None),
        ],
    );

    (input, mlp)
}

pub fn train_model(input: &NodeRef, mlp: &Mlp, epochs: usize, batch_size: usize, step: f32) {
    let train_dataset = build_dataset(500);
    let test_dataset = build_dataset(50);
    //let train_dataset = build_dataset(50);
    //let test_dataset = build_dataset(15);

    let output = mlp.outputs().get(0).unwrap().clone();
    output.set_label("output");
    let batch_size_node = NodeRef::from(batch_size as f32);
    let expected_node = NodeRef::from(0.0);
    let loss = NodeRef::chained(
        &[
            batch_size_node.clone(),
            expected_node.clone(),
            output.clone(),
        ],
        &MseOp,
    );

    println!();
    println!("Starting training for model SinePredictor...");
    println!(
        "Params: epochs={} batch_size={} step={}",
        epochs, batch_size, step
    );
    println!();

    for epoch in 0..epochs {
        println!("Training Epoch {}", epoch);

        let mut batch_start = 0usize;
        let mut batch_index = 0usize;
        while batch_start < train_dataset.len() - 1 {
            let batch_end = (batch_start + batch_size).min(train_dataset.len());
            let batch = train_dataset[batch_start..batch_end].iter();

            loss.clear_gradients();

            let mut total_loss = 0.0;
            for (input_value, expected) in batch {
                input.set_value(*input_value);
                expected_node.set_value(*expected);

                loss.compute_value();
                loss.compute_gradients();

                //println!(" training input={} output={}", input_value, output.value());

                //for param in mlp.parameters() {
                //    println!("{}", param);
                //}
                //panic!("Halt!");
                //
                //println!("actual {} expected {}", output.value(), *expected);

                total_loss += loss.value();
            }

            println!("  batch {} loss {}", batch_index, total_loss);

            for parameter in mlp.parameters() {
                let param = parameter.value();
                let grad = parameter.gradient();
                let adjusted = param - (grad * step);

                //println!("Adjusting param {}, {}", parameter, adjusted);
                parameter.set_value(adjusted);
            }

            batch_start += batch_size;
            batch_index += 1;
        }
    }

    println!("Training complete!");
    println!();

    println!("Starting validation...");

    let mut testing_loss = 0.0;
    for (input_value, expected) in test_dataset {
        expected_node.set_value(expected);
        input.set_value(input_value);

        loss.compute_value();
        testing_loss += loss.value();
        //println!(" validation output={}", output.value());
    }

    println!("  loss={}", testing_loss);
    println!();
}

pub fn test_model(input: &NodeRef, mlp: &Mlp) {
    let dataset_chunks = 10;
    let datapoints_per_chunk = 20;
    let mut dataset = Vec::<(f32, f32)>::with_capacity(dataset_chunks * datapoints_per_chunk);

    for chunk in 0..dataset_chunks {
        for point in 0..datapoints_per_chunk {
            let x = chunk as f32 + point as f32 / datapoints_per_chunk as f32;
            let y = x.sin();
            dataset.push((x, y));
        }
    }

    println!("Starting testing...");
    println!("  dataset_length={}", dataset.len());

    for (input_value, expected_value) in dataset {
        input.set_value(input_value);

        let output = mlp.outputs().get(0).unwrap().clone();
        output.compute_value();
        let actual_value = output.value();

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
            let y = x.sin();
            (x, y)
        })
        .collect::<Vec<_>>()
}
