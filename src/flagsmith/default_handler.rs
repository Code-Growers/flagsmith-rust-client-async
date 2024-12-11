use crate::Flag;

pub trait DefaultHandler {
    fn get_default(&self, feature_name: &str) -> Flag;
}
