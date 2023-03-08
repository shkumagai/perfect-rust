//!
//! # 3章 変数と定数
//! サンプルコード
//!

/// ### 3-1.変数と定数
/// #### リスト３．１ 変数宣言
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn declare_variables(){
    let x: i32 = 0;     // 整数型変数を０で初期化して宣言
    let y = 100;        // 整数型変数を１００で初期化して宣言（型推論）
}

/// ### 3-1.変数と定数
/// #### リスト３．２ ミュータブルな変数の宣言
#[allow(dead_code)]
pub fn declare_mutable_variables(){
    let mut x = 100;    // 変更可能な変数を宣言
    x = x + 100;        // 変数に加算結果を代入して変更する
    println!("x = {}", x);
}

/// ### 3-1.変数と定数
/// #### リスト3.3 シャドーイング
#[allow(dead_code)]
pub fn shadowing(){
    // 変数の上書き
    let value1: i32 = 100;  // 変数value1（i32型）を宣言する
    println!("value1 = {}", value1);
    let value1: i32 = 10;   // 変数が上書きされる
    println!("value1 = {}", value1);

    // シャドーイング
    let value2: f32 = 100.1;    // 変数value2（f32型）を宣言する
    println!("value2 = {}", value2);
    let value2: i32 = 100;      // 変数value1（i32型）を宣言する
    println!("value2 = {}", value2);
}
