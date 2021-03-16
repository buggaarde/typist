pub trait Layout: Default {}

pub struct PlanckEZ {}

impl Default for PlanckEZ {
    fn default() -> Self {
        PlanckEZ {}
    }
}
impl Layout for PlanckEZ {}
