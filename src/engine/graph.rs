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
