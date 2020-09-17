use crate::flow::provider::Provider;
use crate::flow::rf::Rf;
use crate::flow::slot::Slot;

pub type NodeRef = Rf<Node>;

pub struct Node {
    pub slots: Vec<Rf<Slot>>,
    pub providers: Vec<Rf<Provider>>,
    inner: Box<dyn NodeInner>,
}

pub trait NodeInner {
    fn new() -> Self
    where
        Self: Sized;
    fn get_slots(self: &Self) -> Vec<Rf<Slot>>;
    fn get_providers(self: &Self) -> Vec<Rf<Provider>>;
    fn run(self: &mut Self);
}

impl Node {
    pub fn new<T: 'static + NodeInner>() -> NodeRef {
        let inner = Box::new(T::new());
        let rf = Rf::new(Node {
            slots: inner.get_slots(),
            providers: inner.get_providers(),
            inner,
        });
        {
            let rf_mut = &rf.borrow_mut();
            for provider in &rf_mut.providers {
                provider.borrow_mut().owner = rf.downgrade();
            }
            for slot in &rf_mut.slots {
                slot.borrow_mut().owner = rf.downgrade();
            }
        }
        rf
    }

    pub fn run(&mut self) {
        self.inner.run();
    }
}