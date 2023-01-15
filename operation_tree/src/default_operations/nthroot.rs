use crate::op_node::OpNode;

pub struct NthrtNode{
  nth: Box<dyn OpNode<f64>>,
  base: Box<dyn OpNode<f64>>
}

impl NthrtNode{
  pub fn new(nth: Box<dyn OpNode<f64>>, base: Box<dyn OpNode<f64>>) -> Self{
    Self {nth, base}
  }
}

impl OpNode<f64> for NthrtNode{
  fn operate(&self) -> f64 {
    self.base.operate().powf(1.0 / self.nth.operate())
  }
}

#[cfg(test)]
mod tests_for_nthrt_node{
  use crate::op_node::{default::value::ValueNode, OpNode};
  use super::NthrtNode;

  #[test]
  fn testing_operate(){
    let nthrt = NthrtNode::new(
      Box::new(ValueNode::new(4.0)),
      Box::new(ValueNode::new(81.0))
    );
    assert_eq!(nthrt.operate(), 3.0)
  }

  #[test]
  fn testing_operate_return_nan(){
    let nthrt = NthrtNode::new(
      Box::new(ValueNode::new(4.0)),
      Box::new(ValueNode::new(-1.0))
    );
    assert!(nthrt.operate().is_nan())
  }
}