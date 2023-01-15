use std::ops::Add;
use crate::op_node::OpNode;

pub struct AddNode<T: Add<Output = T>>{
  addend_1: Box<dyn OpNode<T>>,
  addend_2: Box<dyn OpNode<T>>
}

impl<T> AddNode<T>
where T: Add<Output = T> + Copy{
  pub fn new(addend_1: Box<dyn OpNode<T>>, addend_2: Box<dyn OpNode<T>>) -> Self{
    Self { addend_1, addend_2 }
  }
}

impl<T> OpNode<T> for AddNode<T>
where T: Add<Output = T> + Copy {
  fn operate(&self) -> T {
    self.addend_1.operate() + self.addend_2.operate()
  }
}

#[cfg(test)]
mod test_for_add_node{
  use super::AddNode;
  use crate::{op_node::default::value::ValueNode, op_node::OpNode};

  #[test]
  fn testing_operate(){
    let add_node = AddNode::new(
      Box::new(ValueNode::new(9.0)),
      Box::new(ValueNode::new(10.0))
    );

    assert_eq!(add_node.operate(), 19.0);
  }
}