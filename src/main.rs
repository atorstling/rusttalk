extern crate crossbeam;

fn pcons<F1, R1, F2, R2>(f1: F1, f2: F2) -> (R1, R2)
    where F1: FnOnce() -> R1 + Send,
          F2: FnOnce() -> R2 + Send,
          R1: Send,
          R2: Send
{
    crossbeam::scope(|scope| {
        (scope.spawn(f1).join(), scope.spawn(f2).join())
    })
}

#[test]
fn pcons_returns_correct_values() {
    let (a, b) = pcons(|| 1, || 2);
    assert!(a == 1);
    assert!(b == 2);
}

#[test]
fn pcons_can_be_chained() {
    let (a, (b, c)) = pcons(|| 1, || pcons(|| 2, || 3));
    assert!(a == 1);
    assert!(b == 2);
    assert!(c == 3);
}
