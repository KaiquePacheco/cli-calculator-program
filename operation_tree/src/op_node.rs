/// # OpNode
/// This trait implements the operate function to some struct.
/// 
/// - T: Represents the type which the operate function is going to return.
/// ``` rust
/// trait OpNode<T>{
///   fn operate(&self) -> T;
/// }
/// struct Value{
///   value: f64
/// }
/// impl OpNode<f64> for Value{
///   fn operate(&self) -> f64{
///     self.value
///   }
/// }
/// 
/// let v = Value{value:80.0};
/// assert_eq!(v.operate(), 80.0);
/// ```
pub trait OpNode<T>{
  fn operate(&self) -> T; 
}

#[cfg(test)]
mod op_node_test{
  use super::OpNode;

  struct Value{
    value: f64
  }

  impl OpNode<f64> for Value{
    fn operate(&self) -> f64 {
      self.value
    }
  }

  #[test]
  fn test_value(){
    let value = Value{value: 10.0};

    assert_eq!(value.operate(), 10.0);
  }
}