mod layout;
mod optimization;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_compiles() {
        use optimization::{self, Criterion, Method};

        let planck_ez = layout::PlanckEZ::default();
        let optimizer = optimization::OptimizationBuilder::new()
            .layout(planck_ez)
            .method(Method::Annealing)
            .criteria(vec![
                Criterion::HandAlternationFactor(1.4),
                Criterion::CountSingleHandStreaksAcrossLeftRightSwitches,
            ])
            .build();

        optimizer.optimize();
    }
}
