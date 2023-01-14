use std::ops::Mul;
use crate::op_node::OpNode;

pub struct MulNode<T>
where T: Copy + Mul<Output = T>{
  factor_1: Box<dyn OpNode<T>>,
  factor_2: Box<dyn OpNode<T>>
}

impl<T> MulNode<T>
where T: Mul<Output = T> + Copy{
  pub fn new(factor_1: Box<dyn OpNode<T>>, factor_2: Box<dyn OpNode<T>>) -> Self{
    Self{factor_1, factor_2}
  }
}

impl<T> OpNode<T> for MulNode<T>
where T: Copy + Mul<Output = T>{
  fn operate(&self) -> T{
    self.factor_1.operate() * self.factor_2.operate()
  }
}

#[cfg(test)]
mod test_for_mul_node{
  use crate::{default_operations::value::ValueNode, op_node::OpNode};
  use super::MulNode;

  #[test]
  fn testing_operate(){
    let mul = MulNode::new(
      Box::new(ValueNode::new(8.0)),
      Box::new(ValueNode::new(10.0))
    );

    assert_eq!(mul.operate(), 80.0);
  }
}