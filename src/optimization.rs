use crate::layout;
use std::mem;

pub struct Optimizer<L: layout::Layout> {
    layout: L,
    method: Method,
    criteria: Vec<Criterion>,
}

impl<L: layout::Layout> Optimizer<L> {
    pub fn optimize(self) {}
}

impl<L: layout::Layout> Default for Optimizer<L> {
    fn default() -> Self {
        Optimizer::<L> {
            layout: L::default(),
            method: Method::Annealing,
            criteria: vec![],
        }
    }
}

impl<L: layout::Layout> Optimizer<L> {
    fn layout(&mut self, _layout: L) {
        self.layout = L::default();
    }

    fn method(&mut self, method: Method) {
        self.method = method;
    }

    fn criteria(&mut self, criteria: Vec<Criterion>) {
        self.criteria = criteria;
    }
}

pub struct OptimizationBuilder<L: layout::Layout> {
    optimizer: Optimizer<L>,
}

impl<L: layout::Layout> Default for OptimizationBuilder<L> {
    fn default() -> Self {
        OptimizationBuilder::<L> {
            optimizer: Optimizer::<L>::default(),
        }
    }
}

impl<L: layout::Layout> OptimizationBuilder<L> {
    pub fn new() -> Self {
        OptimizationBuilder::default()
    }

    pub fn layout(&mut self, layout: L) -> &mut Self {
        self.optimizer.layout(layout);
        self
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.optimizer.method(method);
        self
    }

    pub fn criteria(&mut self, criteria: Vec<Criterion>) -> &mut Self {
        self.optimizer.criteria(criteria);
        self
    }

    pub fn build(&mut self) -> Optimizer<L> {
        let optimizer = mem::take(&mut self.optimizer);
        Optimizer::<L> { ..optimizer }
    }
}

pub enum Method {
    Annealing,
}
pub enum Criterion {
    HandAlternationFactor(f64),
    CountSingleHandStreaksAcrossLeftRightSwitches,
}
