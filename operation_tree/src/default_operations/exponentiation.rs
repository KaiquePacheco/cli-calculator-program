use crate::op_node::OpNode;

pub struct ExpNode{
  base: Box<dyn OpNode<f64>>,
  power: Box<dyn OpNode<f64>>
}

impl ExpNode{
  pub fn new(base: Box<dyn OpNode<f64>>, power: Box<dyn OpNode<f64>>) -> Self{
    Self{base, power}
  }
}

impl OpNode<f64> for ExpNode{
  fn operate(&self) -> f64 {
    self
      .base
      .operate()
      .powf(
        self
        .power
        .operate()
      )
  }
}

#[cfg(test)]
mod tests_for_exp_node{
  use crate::{default_operations::value::ValueNode, op_node::OpNode};
  use super::ExpNode;

  #[test]
  fn testing_operate(){
    let exp = ExpNode::new(
      Box::new(ValueNode::new(2.0)),
      Box::new(ValueNode::new(3.0))
    );

    assert_eq!(exp.operate(), 8.0);
  }

  #[test] 
  fn testing_operate_return_nan(){
    let exp = ExpNode::new(
      Box::new(ValueNode::new(-4.0)),
      Box::new(ValueNode::new(0.5))
    );

    assert!(exp.operate().is_nan());
  }
}