use std::ops::Sub;
use crate::op_node::OpNode;

pub struct SubNode<T>
where T: Copy + Sub<Output = T>{
  minuend: Box<dyn OpNode<T>>,
  subtrahend: Box<dyn OpNode<T>>
}

impl<T> SubNode<T>
where T: Copy + Sub<Output = T>{
  pub fn new(minuend: Box<dyn OpNode<T>>, subtrahend: Box<dyn OpNode<T>>) -> Self{
    Self {minuend, subtrahend}
  }
}

impl<T> OpNode<T> for SubNode<T> 
where T: Copy + Sub<Output = T>{
  fn operate(&self) -> T{
    self.minuend.operate() - self.subtrahend.operate()
  }
}

#[cfg(test)]
mod tests_for_sub_node{
  use super::SubNode;
  use crate::op_node::{OpNode, default::value::ValueNode};

  #[test]
  fn testing_operate(){
    let sub = SubNode::new(
      Box::new(ValueNode::new(90.0)),
      Box::new(ValueNode::new(20.0))
    );

    assert_eq!(sub.operate(), 70.0);
  }
}