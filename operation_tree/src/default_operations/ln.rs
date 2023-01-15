use crate::op_node::OpNode;

pub struct LnNode{
  logarithmend: Box<dyn OpNode<f64>>
}

impl LnNode{
  pub fn new(logarithmend: Box<dyn OpNode<f64>>) -> Self{
    Self { logarithmend }
  }
}

impl OpNode<f64> for LnNode{
  fn operate(&self) -> f64 {
    self.logarithmend.operate().ln()
  }
}

#[cfg(test)]
mod tests_for_ln_node{
  use std::f32::consts::E;
  use crate::{default_operations::value::ValueNode, op_node::OpNode};
  use super::LnNode;

  #[test]
  fn testing_operate(){
    let ln = LnNode::new(
      Box::new(ValueNode::new(E.powi(3) as f64))
    );
    
    assert_eq!(ln.operate() as i64 + 1, 3)
  }

  #[test]
  fn testing_operate_return_nan(){
    let ln = LnNode::new(
      Box::new(ValueNode::new(-(E.powi(3) as f64)))
    );

    assert!(ln.operate().is_nan())
  }
}