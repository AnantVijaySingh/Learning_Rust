#[derive(Debug)]
pub struct Asparagus {
    pub(crate) name: String,
    pub(crate) nutritional_value: NutritionalValue
}

#[derive(Debug)]
pub enum NutritionalValue {
    High,
    Medium,
    Low
}