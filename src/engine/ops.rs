use crate::engine::{Closure, NodeRef, Op, WeakNodeRef};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct AddOp;

impl Op for AddOp {
    fn min_inputs(&self) -> usize {
        2
    }

    fn name(&self) -> &str {
        "add"
    }

    fn forward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let inputs = inputs.to_vec();
        Rc::new(move || {
            let mut sum = 0.0;
            for input in &inputs {
                sum += input.value();
            }

            let output = output.upgrade().unwrap();
            (*output.borrow_mut()).val = sum;
        })
    }

    fn backward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let inputs = inputs.to_vec();
        Rc::new(move || {
            let output = output.upgrade().unwrap();
            let out_grad = output.borrow().grad;

            for input in &inputs {
                (*input.borrow_mut()).grad += out_grad;
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct MulOp;

impl Op for MulOp {
    fn min_inputs(&self) -> usize {
        2
    }

    fn name(&self) -> &str {
        "mul"
    }

    fn forward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let inputs = inputs.to_vec();
        Rc::new(move || {
            let mut sum = 0.0;
            for (index, input) in inputs.iter().enumerate() {
                if index == 0 {
                    sum = input.value();
                } else {
                    sum *= input.value();
                }
            }

            let output = output.upgrade().unwrap();
            (*output.borrow_mut()).val = sum;
        })
    }

    fn backward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let inputs = inputs.to_vec();
        Rc::new(move || {
            let output = output.upgrade().unwrap();
            let out_grad = (*output.borrow()).grad;
            let mulled = inputs
                .iter()
                .map(|i| i.value())
                .reduce(|acc, v| acc * v)
                .unwrap_or(0.0);

            for input in &inputs {
                let value = input.value();
                let mulled_without_value = if value == 0.0 && mulled == 0.0 {
                    0.0
                } else {
                    mulled / value
                };
                (*input.borrow_mut()).grad += mulled_without_value * out_grad;
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct ExpOp;

impl Op for ExpOp {
    fn min_inputs(&self) -> usize {
        2
    }

    fn max_inputs(&self) -> usize {
        2
    }

    fn name(&self) -> &str {
        "exp"
    }

    fn forward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let inputs = inputs.to_vec();
        Rc::new(move || {
            let base = inputs[0].value();
            let exponent = inputs[1].value();
            let powered = base.powf(exponent);

            let output = output.upgrade().unwrap();
            (*output.borrow_mut()).val = powered;
        })
    }

    fn backward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let inputs = inputs.to_vec();
        Rc::new(move || {
            let output_node = output.upgrade().unwrap();
            let output_value = (output_node.borrow()).val;
            let output_grad = (output_node.borrow()).grad;

            let base = inputs[0].value();
            let exponent = inputs[1].value();

            (*inputs[0].borrow_mut()).grad = output_value.ln() * base.powf(exponent) * output_grad;
            (*inputs[1].borrow_mut()).grad = (exponent * base).powf(exponent - 1.0) * output_grad;
        })
    }
}

#[derive(Clone, Debug)]
pub struct ReluOp;

impl Op for ReluOp {
    fn min_inputs(&self) -> usize {
        1
    }

    fn max_inputs(&self) -> usize {
        1
    }

    fn name(&self) -> &str {
        "relu"
    }

    fn forward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let input = inputs.get(0).unwrap().clone();
        Rc::new(move || {
            let raw_value = input.value().clone();
            let clamped_value = if raw_value <= 0.0 { 0.0 } else { raw_value };
            let output = output.upgrade().unwrap();

            (*output.borrow_mut()).val = clamped_value;
        })
    }

    fn backward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let input = inputs.get(0).unwrap().clone();
        Rc::new(move || {
            let output = output.upgrade().unwrap();
            let out_grad = (*output.borrow()).grad;
            let out_val = (*output.borrow()).val;

            (*input.borrow_mut()).grad += if out_val <= 0.0 { 0.0 } else { out_grad };
        })
    }
}

#[derive(Clone, Debug)]
pub struct TanhOp;

impl Op for TanhOp {
    fn min_inputs(&self) -> usize {
        1
    }
    fn max_inputs(&self) -> usize {
        1
    }

    fn name(&self) -> &str {
        "tanh"
    }

    fn forward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let input = inputs.get(0).unwrap().clone();
        Rc::new(move || {
            let output = output.upgrade().unwrap();

            (*output.borrow_mut()).val = input.value().tanh();
        })
    }

    fn backward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let input = inputs.get(0).unwrap().clone();
        Rc::new(move || {
            let output = output.upgrade().unwrap();
            let out_grad = (*output.borrow()).grad;
            let out_val = (*output.borrow()).val;

            (*input.borrow_mut()).grad += (1.0 - out_val.powi(2)) * out_grad;
        })
    }
}

#[derive(Clone, Debug)]
pub struct MseOp;

impl Op for MseOp {
    fn min_inputs(&self) -> usize {
        3
    }
    fn max_inputs(&self) -> usize {
        3
    }

    fn name(&self) -> &str {
        "mse"
    }
    fn forward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let total = inputs.get(0).unwrap().clone();
        let expected = inputs.get(1).unwrap().clone();
        let actual = inputs.get(2).unwrap().clone();
        Rc::new(move || {
            let total_val = total.value();
            let value = (expected.value() - actual.value()).powi(2) / total_val;
            let output = output.upgrade().unwrap();

            (*output.borrow_mut()).val = value;
        })
    }

    fn backward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure {
        self.validate_inputs_length(inputs.len());

        let total = inputs.get(0).unwrap().clone();
        let expected = inputs.get(1).unwrap().clone();
        let actual = inputs.get(2).unwrap().clone();
        Rc::new(move || {
            let total_val = total.value();
            let output = output.upgrade().unwrap();
            let out_grad = (*output.borrow()).grad;
            let expected_val = expected.value();
            let actual_val = actual.value();

            // dL/dY = (2/n)(Y - y)
            // dL/dy = (-2/n)(Y - y)
            let error = expected_val - actual_val;
            (*expected.borrow_mut()).grad += (2.0 / total_val) * error * out_grad;
            (*actual.borrow_mut()).grad += (-2.0 / total_val) * error * out_grad;
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_op_computes_nested_values() {
        let a = NodeRef::from(10.0);
        let b = NodeRef::from(5.0);
        let c = NodeRef::chained(&[a.clone(), b.clone()], &AddOp);
        let d = NodeRef::from(6.0);
        let e = NodeRef::chained(&[c.clone(), d.clone()], &AddOp);
        e.compute_value();

        assert_eq!(e.value(), 21.0);
    }

    #[test]
    fn mul_op_computes_nested_values() {
        let a = NodeRef::from(2.0);
        let b = NodeRef::from(3.0);
        let c = NodeRef::chained(&[a.clone(), b.clone()], &MulOp);
        let d = NodeRef::from(4.0);
        let e = NodeRef::chained(&[c.clone(), d.clone()], &MulOp);
        e.compute_value();

        assert_eq!(e.value(), 24.0);
        assert_eq!(c.value(), 6.0);
    }

    #[test]
    fn mul_op_backpropogates_gradients() {
        let a = NodeRef::from(2.0);
        let b = NodeRef::from(3.0);
        let c = NodeRef::chained(&[a.clone(), b.clone()], &MulOp);
        c.compute_value();
        c.compute_gradients();

        assert_eq!(c.gradient(), 1.0);
        assert_eq!(b.gradient(), 2.0);
        assert_eq!(a.gradient(), 3.0);
    }

    #[test]
    fn mul_op_backpropogates_nested_gradients() {
        let a = NodeRef::from(2.0);
        let b = NodeRef::from(3.0);
        let c = NodeRef::chained(&[a.clone(), b.clone()], &MulOp);
        let d = NodeRef::from(4.0);
        let e = NodeRef::chained(&[c.clone(), d.clone()], &MulOp);
        e.compute_value();
        e.compute_gradients();

        assert_eq!(e.gradient(), 1.0);
        assert_eq!(d.gradient(), 6.0);
        assert_eq!(c.gradient(), 4.0);
        assert_eq!(b.gradient(), 8.0);
        assert_eq!(a.gradient(), 12.0);
    }
}
