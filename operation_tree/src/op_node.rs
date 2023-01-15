#[path ="default.rs"]
mod default;

pub trait OpNode<T>
where T: Copy{
  fn operate(&self) -> T; 
}

#[cfg(test)]
mod op_node_test{
  use super::OpNode;

  struct Mock{
    value: f64
  }

  impl OpNode<f64> for Mock{
    fn operate(&self) -> f64 {
      self.value
    }
  }

  #[test]
  fn test_value(){
    let value = Mock{value: 10.0};

    assert_eq!(value.operate(), 10.0);
  }
}