#![feature(fnbox)]
#![feature(closure_to_fn_coercion)]
extern crate crossbeam;
use std::boxed::FnBox;

// Crossbeam::scope requires Send:
// fn spawn<F, T>(&self, f: F) -> ScopedJoinHandle<T> 
// where F: FnOnce() -> T + Send + 'a, T: Send + 'a
// Send means Sync + Copy
pub fn pcons<F1, R1, F2, R2>(f1: F1, f2: F2) -> (R1, R2)
    where F1: FnOnce() -> R1 + Send,
          F2: FnOnce() -> R2 + Send,
          R1: Send,
          R2: Send 
{
    crossbeam::scope(|scope| {
        (scope.spawn(f1)
            .join(),
         scope.spawn(f2).join())
    })
}


#[test]
fn pcons_returns_correct_values() {
    let (a, b) = pcons(|| 1, || 2);
    assert_eq!(a,1);
    assert_eq!(b,2);
}

#[test]
fn pcons_can_be_chained() {
    let (a, (b, c)) = pcons(|| 1, || pcons(|| 2, || 3));
    assert_eq!(a,1);
    assert_eq!(b,2);
    assert_eq!(c,3);
}

// If we use &[&F], we can pass the closure, but
// cannot call it since the size is unknown, see
// http://stackoverflow.com/questions/30411594/moving-a-boxed-function

// &[Box<F>] is also unusable for the same reason, Box<FnOnce> is
// ususable.

// We cannot use &[F], since F is unsized
// note: slice and array elements must have `Sized` type

//
// where F: FnBox() -> R 

// Goal: have pconsl use pcons
//
pub fn pconsl<F, R>(mut fs: Vec<F>) -> Vec<R>
  where F: FnOnce() -> R + Send + Sync,
        R: Send ,
{
        if fs.len() == 0 {
                let vec: Vec<R> = vec![];
                return vec; }
        if fs.len() == 1 {
                return vec![fs.remove(0)()];
            }
      let tail: Vec<F> = fs.split_off(1);
      let head: F = fs.remove(0);
        let mut res: (R, Vec<R>) = pcons(
             || head(),
             || pconsl(tail)
            );
        let mut arr: Vec<R> = Vec::new();
        arr.push(res.0);
        arr.append(&mut res.1);
        arr
        }


pub fn pconsl2<F, R>(mut fs: Vec<Box<F>>) -> Vec<R>
  where F: FnBox() -> R + Send + ?Sized,
        R: Send ,
{
    if fs.len() == 0 {
        let vec: Vec<R> = vec![];
        return vec;
    }
    if fs.len() == 1 {
        let head: Box<F> = fs.remove(0);
        return vec![head.call_box(())];
    }
  let tail: Vec<Box<F>> = fs.split_off(1);
  let head: Box<F> = fs.remove(0);
    let mut res: (R, Vec<R>) = pcons(
     || head.call_box(()),
     || pconsl2(tail)
    );
    let mut arr: Vec<R> = Vec::new();
    arr.push(res.0);
    arr.append(&mut res.1);
    arr
}
// fn plist<F, R>(fs: &[&F]) -> &
#[test]
fn pconsl_works() {
    let a = || 1;
    let b = || 2;
    let arr: Vec<fn() -> u32> = vec![a, b];
    //arr.push(&a);
    //arr.push(&b);
    let res = pconsl(arr);
    assert_eq!(res.get(0).unwrap(), &1);
    //assert_eq!(res.get(0).unwrap(),&String::from("a"));
}

