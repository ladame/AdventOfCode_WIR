use day_06_lib::guard::Guard;

#[test]
fn test_guard_new(){
    let guard: Guard = Guard::new(1,2);
    assert_eq!(guard.get_position(), (1,2));

}

#[test]
fn test_guard_right(){
    let guard: Guard = Guard::new(6,4);
    let guard_right: Guard = guard.right();
    assert_eq!(guard_right.get_position(), (4,-6));
}

#[test]
fn test_guard_add(){
    let guard: Guard = Guard::new(6,4);
    let dir: Guard = Guard::new(-1,0);
    let guard: Guard = guard.add(&dir);
    assert_eq!(guard.get_position(), (5,4));
}