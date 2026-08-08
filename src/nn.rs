use crate::graph::{AddOp, MulOp, NodeRef, Op};
use rand::prelude::*;

pub struct Neuron {
    bias: NodeRef,
    weights: Vec<NodeRef>,
    output: NodeRef,
}

impl Neuron {
    pub fn new(inputs: &[NodeRef], activation: Option<&dyn Op>) -> Self {
        let total_inputs = inputs.len();
        if total_inputs == 0 {
            panic!(
                "Neuron::new must have at least 1 input, got {}",
                total_inputs
            );
        }

        let mut weights_by_inputs = Vec::<NodeRef>::with_capacity(total_inputs);
        let mut weights = Vec::<NodeRef>::with_capacity(total_inputs);
        for input in inputs {
            let random_value = rand::random_range(-1.0..=1.0);
            let weight = NodeRef::from(random_value);
            weight.set_label("w");

            let weight_by_input = NodeRef::chained(&[weight.clone(), input.clone()], &MulOp);
            weight_by_input.set_label("wi");

            weights.push(weight);
            weights_by_inputs.push(weight_by_input);
        }

        let bias = NodeRef::from(0.0);
        bias.set_label("b");

        let summed_inputs = if total_inputs == 1 {
            weights_by_inputs.get(0).unwrap().clone()
        } else {
            NodeRef::chained(&weights_by_inputs, &AddOp)
        };
        let summed_bias = NodeRef::chained(&[summed_inputs.clone(), bias.clone()], &AddOp);

        let output = match activation {
            Some(op) => NodeRef::chained(&[summed_bias], op),
            _ => summed_bias,
        };
        output.set_label("a");

        Self {
            bias,
            weights,
            output,
        }
    }

    pub fn parameters(&self) -> Vec<NodeRef> {
        let mut params = vec![];
        params.extend_from_slice(&self.weights);
        params.push(self.bias.clone());
        params
    }

    pub fn output(&self) -> NodeRef {
        self.output.clone()
    }

    pub fn set_bias(&self, bias: f32) {
        (*self.bias.borrow_mut()).val = bias;
    }

    pub fn set_weights(&self, weights: &[f32]) {
        for (value, weight) in weights.iter().zip(self.weights.iter()) {
            (*weight.borrow_mut()).val = *value;
        }
    }
}

pub struct Dense {
    neurons: Vec<Neuron>,
}

impl Dense {
    pub fn new(inputs: &[NodeRef], total_outputs: usize, activation: Option<&dyn Op>) -> Self {
        let total_inputs = inputs.len();
        if total_inputs == 0 || total_outputs == 0 {
            panic!(
                "Expected Dense::new to have at least 1 input and output each, got {} and {} respectively",
                total_inputs, total_outputs
            );
        }

        let mut neurons = Vec::with_capacity(total_outputs);
        for _ in 0..total_outputs {
            let neuron = Neuron::new(inputs, activation.clone());
            neurons.push(neuron);
        }

        Self { neurons }
    }

    pub fn outputs(&self) -> Vec<NodeRef> {
        self.neurons.iter().map(|n| n.output()).collect::<Vec<_>>()
    }

    pub fn parameters(&self) -> Vec<NodeRef> {
        let mut params = vec![];
        for neuron in self.neurons.iter() {
            let inner_params = neuron.parameters();
            params.extend_from_slice(&inner_params);
        }
        params
    }
}

pub struct Mlp {
    layers: Vec<Dense>,
}

/// Number of outputs and activation function of the dense layer
pub type MlpDenseOpts<'a> = (usize, Option<&'a dyn Op>);

impl Mlp {
    pub fn new(input: NodeRef, layers: &'static [MlpDenseOpts]) -> Self {
        let total_layers = layers.len();
        if total_layers == 0 {
            panic!(
                "Expected Mlp::new to have more than 1 layer, got {}",
                total_layers
            );
        }

        let mut denses = Vec::<Dense>::with_capacity(total_layers);
        for index in 0..total_layers {
            let (total_outputs, activation) = layers[index];
            let dense = if index == 0 {
                Dense::new(&[input.clone()], total_outputs, activation)
            } else {
                let last_dense = &denses[index - 1];
                Dense::new(&last_dense.outputs(), total_outputs, activation)
            };

            denses.push(dense);
        }

        Self { layers: denses }
    }

    pub fn parameters(&self) -> Vec<NodeRef> {
        let mut params = vec![];
        for layer in self.layers.iter() {
            let inner_params = layer.parameters();
            params.extend_from_slice(&inner_params);
        }
        params
    }

    pub fn outputs(&self) -> Vec<NodeRef> {
        let last_layer = &self.layers[self.layers.len() - 1];
        last_layer.outputs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neuron_without_activation_computes_correct_output() {
        let input = NodeRef::from(10.0);

        let neuron = Neuron::new(&[input], None);
        neuron.set_bias(4.5);
        neuron.set_weights(&[3.0]);

        let output = neuron.output();
        output.compute_value();

        assert_eq!(output.value(), 34.5)
    }
}
