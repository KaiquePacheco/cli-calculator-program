use crate::op_node::OpNode;

pub struct ValueNode<T>
where T: Copy{
  value: T
}

impl<T> ValueNode<T>
where T: Copy{
  pub fn new(value: T) -> Self{
    Self { value }
  }
}

impl<T> OpNode<T> for ValueNode<T>
where T: Copy{
  fn operate(&self) -> T {
    self.value
  }
}

#[cfg(test)]
mod testing_value{
  use crate::op_node::OpNode;
  use super::ValueNode;

  #[test]
  fn test_value(){
    let value = ValueNode::new(90.0);
    assert_eq!(value.operate(), 90.0);
  }
}