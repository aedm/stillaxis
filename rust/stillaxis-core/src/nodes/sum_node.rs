use crate::node::Node;
use crate::node_descriptor::{NodeDescriptor, NodeId};
use crate::providers::float_provider::FloatProvider;
use crate::slots::float_slot::FloatSlot;
use std::fmt;

pub struct SumNode {
    descriptor: NodeDescriptor,
    pub a: FloatSlot,
    pub b: FloatSlot,
    pub sum: FloatProvider,
}

impl Node for SumNode {
    fn new(id: NodeId) -> SumNode {
        let a = FloatSlot::new("a");
        let b = FloatSlot::new("b");
        let sum = FloatProvider::new("sum");
        let slots = vec![a.slot.clone(), b.slot.clone()];
        let providers = vec![sum.provider.clone()];
        SumNode {
            descriptor: NodeDescriptor::new(id, "sum_floats", slots, providers),
            a,
            b,
            sum,
        }
    }

    fn descriptor(&self) -> &NodeDescriptor {
        &self.descriptor
    }

    fn descriptor_mut(&mut self) -> &mut NodeDescriptor {
        &mut self.descriptor
    }

    fn run(self: &mut Self) {
        let result = self.a.get() + self.b.get();
        self.sum.set(result);
    }
}

impl fmt::Debug for SumNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.descriptor.fmt(f)
    }
}
