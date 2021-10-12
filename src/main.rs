mod reset; //include "reset/mod.rs"

fn main() {
      let r = reset::new();
      println!("Hello world!");
      reset::sub1::hello();
      r.print();
}
