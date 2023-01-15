use crate::op_node::OpNode;

pub struct LogNode{
  base: Box<dyn OpNode<f64>>,
  logarithmend: Box<dyn OpNode<f64>>
}

impl LogNode{
  pub fn new(base: Box<dyn OpNode<f64>>, logarithmend: Box<dyn OpNode<f64>>) -> Self{
    Self { base, logarithmend }
  }
}

impl OpNode<f64> for LogNode{
  fn operate(&self) -> f64 {
    self.logarithmend.operate().log(self.base.operate())
  }
}

#[cfg(test)]
mod tests_for_log_node{
  use crate::op_node::{default::value::ValueNode, OpNode};
  use super::LogNode;

  #[test]
  fn testing_operate(){
    let log = LogNode::new(
      Box::new(ValueNode::new(2.0)),
      Box::new(ValueNode::new(8.0))
    );

    assert_eq!(log.operate(), 3.0)
  }

  #[test]
  fn testing_operate_return_nan(){
    let log = LogNode::new(
      Box::new(ValueNode::new(-2.0)),
      Box::new(ValueNode::new(8.0))
    );

    assert!(log.operate().is_nan())
  }
}