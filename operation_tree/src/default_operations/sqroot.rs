use crate::op_node::OpNode;

pub struct SqrtNode{
  base: Box<dyn OpNode<f64>>
}

impl SqrtNode{
  pub fn new(base: Box<dyn OpNode<f64>>) -> Self{
    Self {base}
  }
}

impl OpNode<f64> for SqrtNode{
  fn operate(&self) -> f64 {
    self.base.operate().sqrt()
  }
}

#[cfg(test)]
mod tests_for_sqrt_node{
  use crate::{default_operations::value::ValueNode, op_node::OpNode};
  use super::SqrtNode;

  #[test]
  fn testing_operate(){
    let sqrt = SqrtNode::new(
      Box::new(ValueNode::new(81.0))
    );

    assert_eq!(sqrt.operate(), 9.0);
  }

  #[test]
  fn testing_operate_return_nan(){
    let sqrt = SqrtNode::new(
      Box::new(ValueNode::new(-4.0))
    );

    assert!(sqrt.operate().is_nan());
  }
}