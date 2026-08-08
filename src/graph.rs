use std::{
    cell::RefCell,
    collections::HashSet,
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    rc::{Rc, Weak},
    sync::atomic::{AtomicU64, Ordering},
};

pub trait Op {
    fn min_inputs(&self) -> usize {
        0
    }
    fn max_inputs(&self) -> usize {
        usize::MAX
    }
    fn validate_inputs_length(&self, total_inputs: usize) {
        let max_inputs = self.max_inputs().max(self.min_inputs());
        let min_inputs = self.max_inputs().min(self.min_inputs());

        if total_inputs > max_inputs || total_inputs < min_inputs {
            panic!(
                "Expected operation to have between {} to {} inputs, got {}",
                min_inputs, max_inputs, total_inputs
            );
        }
    }

    fn name(&self) -> &str;
    fn forward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure;
    fn backward(&self, inputs: &[NodeRef], output: WeakNodeRef) -> Closure;
}

pub type WeakNodeRef = Weak<RefCell<Node>>;

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

static NODE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct Node {
    id: u64,
    pub val: f32,
    pub grad: f32,
    pub label: Option<String>,
    activation_label: String,
    forward: Closure,
    backward: Closure,
    parents: Vec<NodeRef>,
}

pub type Closure = Rc<dyn Fn()>;

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match &self.label {
            Some(label) => format!(" {}", label),
            _ => "".to_string(),
        };
        write!(
            f,
            "Node({}{}) val={} grad={} act={}",
            self.id.to_string(),
            name,
            self.val,
            self.grad,
            self.activation_label
        )
    }
}

impl Default for Node {
    fn default() -> Self {
        let id = NODE_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            val: 0.0,
            grad: 0.0,
            label: None,
            activation_label: "_".to_string(),
            forward: Rc::new(|| {}),
            backward: Rc::new(|| {}),
            parents: vec![],
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(hasher);
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct NodeRef(Rc<RefCell<Node>>);

impl fmt::Display for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self
            .try_borrow()
            .expect("NodeRef already borrowed before Display::fmt"))
        .fmt(f)
    }
}

impl Hash for NodeRef {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        (*self.borrow()).hash(hasher);
    }
}

impl Deref for NodeRef {
    type Target = Rc<RefCell<Node>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
impl DerefMut for NodeRef {
    type Target = Rc<RefCell<Node>>;
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
*/

impl From<f32> for NodeRef {
    fn from(value: f32) -> Self {
        let node_ref = Self::new();
        node_ref.set_value(value);
        node_ref
    }
}

impl NodeRef {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Node::default())))
    }

    pub fn chained(inputs: &[NodeRef], op: &dyn Op) -> Self {
        let output = Self::new();
        output.set_parents(inputs.to_vec());
        output.set_operation(
            op.forward(inputs, Rc::downgrade(&output)),
            op.backward(inputs, Rc::downgrade(&output)),
            op.name().to_string(),
        );

        output
    }

    fn set_operation(&self, forward: Closure, backward: Closure, activation_label: String) {
        let mut this = self.borrow_mut();
        (*this).forward = forward;
        (*this).backward = backward;
        (*this).activation_label = activation_label;
    }

    fn set_parents(&self, parents: Vec<Self>) {
        (*self.borrow_mut()).parents = parents;
    }

    pub fn set_gradient(&self, gradient: f32) {
        (*self.borrow_mut()).grad = gradient;
    }

    pub fn set_label<S>(&self, label: S)
    where
        S: Into<String>,
    {
        (*self.borrow_mut()).label = Some(label.into());
    }

    pub fn set_value(&self, value: f32) {
        (*self.borrow_mut()).val = value;
    }

    pub fn compute_value(&self) {
        let ancestry = self.flatten_graph();
        for ancestor in ancestry.iter() {
            ancestor.forward();
        }
    }

    pub fn compute_gradients(&self) {
        self.set_gradient(1.0);
        let ancestry = self.flatten_graph();
        for ancestor in ancestry.iter().rev() {
            ancestor.backward();
        }
    }

    pub fn clear_gradients(&self) {
        let ancestry = self.flatten_graph();
        for ancestor in ancestry.iter().rev() {
            ancestor.set_gradient(0.0);
        }
    }

    pub fn id(&self) -> u64 {
        (*self
            .try_borrow()
            .expect(&format!("{} already borrowed", self)))
        .id
    }

    pub fn value(&self) -> f32 {
        (*self.borrow()).val
    }

    pub fn gradient(&self) -> f32 {
        (*self.borrow()).grad
    }

    fn flatten_graph(&self) -> Vec<Self> {
        let mut visited = HashSet::<u64>::new();
        let mut flattened = Vec::<Self>::new();
        Self::flatten_node(self, &mut visited, &mut flattened);

        flattened
    }

    fn flatten_node<'a>(node: &'a Self, visited: &mut HashSet<u64>, flattened: &mut Vec<Self>) {
        if visited.contains(&node.id()) {
            return;
        }

        let parents = (*node.borrow()).parents.clone();
        for parent in &parents {
            Self::flatten_node(parent, visited, flattened);
        }

        flattened.push(node.clone());
        visited.insert(node.id());
    }

    fn forward(&self) {
        let closure = {
            let node = self.try_borrow().expect(&format!("{}", self));
            let closure = Rc::clone(&node.forward);
            closure
        };
        closure();
    }

    fn backward(&self) {
        let closure = {
            let node = self.try_borrow().expect(&format!("{}", self));
            let closure = Rc::clone(&node.backward);
            closure
        };
        closure();
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
