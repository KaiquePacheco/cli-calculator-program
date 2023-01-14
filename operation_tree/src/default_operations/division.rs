use std::ops::Div;
use crate::op_node::OpNode;

pub struct DivNode<T>
where T: Copy + Div<Output = T>{
  dividend: Box<dyn OpNode<T>>,
  divisor: Box<dyn OpNode<T>>
}

impl<T> DivNode<T> 
where T: Copy + Div<Output = T>{
  fn new(dividend: Box<dyn OpNode<T>>, divisor: Box<dyn OpNode<T>>) -> Self{
    Self { dividend, divisor }
  }
}

impl<T> OpNode<T> for DivNode<T>
where T: Copy + Div<Output = T>{
  fn operate(&self) -> T {
    self.dividend.operate() / self.divisor.operate()
  }
}

#[cfg(test)]
mod tests_for_div_node{
  use crate::{default_operations::value::ValueNode, op_node::OpNode};
  use super::DivNode;

  #[test]
  fn testing_operate(){
    let div = DivNode::new(
      Box::new(ValueNode::new(9.0)), 
      Box::new(ValueNode::new(3.0))
    ); 

    assert_eq!(div.operate(), 3.0);
  }

  #[test]
  #[should_panic]
  fn testing_operate_return_nan(){
    let div = DivNode::new(
      Box::new(ValueNode::new(9.0_f64)), 
      Box::new(ValueNode::new(0.0_f64))
    );
  
    assert!(div.operate().is_nan());
  }
}