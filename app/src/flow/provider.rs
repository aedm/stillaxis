use crate::flow::node::Node;
use crate::flow::rf::{Rf, Weak};
use crate::flow::slot::{Slot};

#[derive(Debug)]
pub enum ProviderValue {
    _None,
    Float32(f32),
}

pub struct Provider {
    pub owner: Weak<Node>,
    pub name: String,
    pub value: ProviderValue,
    pub connections: Vec<Rf<Slot>>,
}

impl Provider {
    fn new(name: &str, value: ProviderValue) -> Provider {
        Provider {
            owner: Weak::new(),
            name: name.to_string(),
            value,
            connections: vec![],
        }
    }
}

pub struct FloatProvider {
    pub provider: Rf<Provider>,
}

impl FloatProvider {
    pub fn new(name: &str) -> FloatProvider {
        FloatProvider {
            provider: Rf::new(Provider::new(name, ProviderValue::Float32(0.0))),
        }
    }

    pub fn set(self: &mut Self, value: f32) {
        self.provider.borrow_mut().value = ProviderValue::Float32(value);
    }
}
