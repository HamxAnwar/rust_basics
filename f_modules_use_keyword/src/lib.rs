mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
    }
}

// Instead of specifying as "front_of_house::hosting::add_to_waitlist();" we can input the use keyword.
// In the below, crate refers to the existing self crate.
// We can use either absolute path or relative path as below respectively.

use crate::front_of_house::hosting;
use self::front_of_house::hosting;

// While in the above, we could have brought the function itself in the use statement but in rust, the way of bringing functions into scope is to bring its parent into scope.

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// While bringing structs, enums or other items into scope, its way is to specify the full path.
// The exception for above is if they have same name but different parents. So we can bring the parent into scope instead of the item so the names doesn't conflict.
// We can also rename the item and skip the parent as below:

//      use std::fmt::Result;
//      use std::io::Result as IoResult;

// Also if we want some external code from this file, want to use the above hosting module, it cannot as the use statement lets the current file use this module. To make it available to external code, we need to make the use statement public too as follows:

//      pub use crate::front_of_house::hosting;

// We can insert the use keyword to import dependencies too such as:

//      use rand::Rng;
//      use rand::CryptoRng;

// We can also combine the above use as:
      
//      use rand::{Rng, CryptoRng};
 
// Also if we want to import as below:
      
//      use std::io;
//      use std::io::Write;

// We can also write the above as:

//      use std::io::{self, Write};

// To bring all the public operators in the scope, we can specify using glob operator.
      
//      use std::io::*;
