extern "C" {
  fn printFoo();
  fn printBar();
}

fn main(){
    unsafe {
        printFoo();
        printBar();
    }
}
