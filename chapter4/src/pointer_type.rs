//!
//! 4章 基本データ型
//! サンプルプログラム
//!

/// ### 4-9.ポインタ型
/// #### リスト4.37 ポインタ型の宣言
#[allow(dead_code)]
pub fn declare() {
    // 整数型、文字列型変数を宣言する
    let x: i32 = 100;
    let y: &str = "ABC";

    // 宣言した変数の生ポインタ（raw pointer）を取得する
    let x_ptr: *const i32 = &x;
    let y_ptr: *const &str = &y;

    // ポインタを利用して値を取得し、出力する
    unsafe {
        println!("x_ptrポインタの値 = {}", *x_ptr);
        println!("y_ptrポインタの値 = {}", *y_ptr);
    }

    // アドレスを出力する
    println!("x_ptrポインタのアドレス = {:?}", x_ptr);
    println!("y_ptrポインタのアドレス = {:?}", y_ptr);
}

/// ### 4-9.ポインタ型
/// #### リスト4.38 値の変更
#[allow(dead_code)]
pub fn mut_declare() {
    // ミュータブルな整数型、文字列型変数を宣言する
    let mut x: i32 = 100;
    let mut y: &str = "ABC";

    // 宣言した変数のポインタ（raw_pointer）を取得する
    let x_ptr: *mut i32 = &mut x;
    let y_ptr: *mut &str = &mut y;

    unsafe {    // 参照外し
        println!("変更前 x_ptrポインタの値 = {}", *x_ptr);
        println!("変更前 y_ptrポインタの値 = {}", *y_ptr);
        *x_ptr += 100;
        let str_val = "ポインタの利用".to_string();
        *y_ptr = &str_val;
        println!("変更後 x_ptrポインタの値 = {}", *x_ptr);
        println!("変更後 y_ptrポインタの値 = {}", *y_ptr);
    }
}

/// ### 4-9.ポインタ型
/// #### リスト4.39 可変ポインタ
#[allow(dead_code)]
pub fn mut_declare_2() {
    let x: i32 = 100;
    let y: i32 = 200;

    // 変数 x のポインタを代入する
    let mut ptr: *const i32 = &x;
    // 変数 ptr を利用する
    unsafe {
        println!("ptr の値 = {}", *ptr);
        println!("ptr のアドレス = {:?}", ptr);
    }
    ptr = &y;   // 変数 y のポインタに変更する
    unsafe {
        println!("ptr の値 = {}", *ptr);
        println!("ptr のアドレス = {:?}", ptr);
    }
}
