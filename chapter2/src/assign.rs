//!
//! 代入演算子のサンプルコード
//!

/// ### リスト２．９
/// #### 代入演算子の利用
#[allow(dead_code)]
pub fn assign_value(x: i64, y: f64){
    let a = x;
    let b = y;
    println!("変数 a の値 = {}", a);
    println!("変数 b の値 = {}", b);
}
/// ### リスト2.10
/// #### 複合代入演算子の利用
#[allow(dead_code)]
pub fn compound_assign(mut x: i32, y: i32){
    x += y;
    println!("x += y = {}", x);
    x -= y;
    println!("x -= y = {}", x);
    x *= y;
    println!("x *= y = {}", x);
    x /= y;
    println!("x /= y = {}", x);
    x %= y;
    println!("x %= y = {}", x);
}
