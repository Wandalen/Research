
#[ cfg( feature = "left" ) ]
#[ cfg( feature = "a" ) ]
pub use left::LEFT_A;
#[ cfg( feature = "left" ) ]
#[ cfg( feature = "b" ) ]
pub use left::LEFT_B;
#[ cfg( feature = "right" ) ]
#[ cfg( feature = "a" ) ]
pub use right::RIGHT_A;
#[ cfg( feature = "right" ) ]
#[ cfg( feature = "b" ) ]
pub use right::RIGHT_B;

use std::sync::OnceLock;
pub static INSTANCE : OnceLock< i32 > = OnceLock::new();
pub const C : i32 = 5;

pub fn f1()
{
  println!( "Hello!" );
}

pub trait Trait1
{
  fn f1()
  {
    println!( "Hello!" );
  }
}

#[ derive( Debug ) ]
pub struct Struct1
{
  pub a : i32,
}

impl Trait1 for Struct1
{
}
