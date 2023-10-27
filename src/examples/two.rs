fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let v = vec![1, 2];
    let res: Option<Vec<_>> = v.iter().map(|x| Some(x * 2)).collect();
    println!("{:?}",res);
    print!("{:?}", six)
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
#[test]
fn test(){
    let v = vec![1, 2];
    let res: Option<Vec<_>> = v.iter().map(|x| Some(x *2)).collect();
    let s :Option<Vec<_>>= Some(vec![2,4]);
    assert_eq!(res,s)
}