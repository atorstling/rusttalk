#[test]
fn borrow() {
  let mut a: u32 = 1;
  let b: &mut u32 = &mut a; 
  println!("{}", a);
}
