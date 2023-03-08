//!
//! # 3章 変数と定数
//! サンプルコード
//!

/// ### ３−１．変数と定数
/// #### リスト3.5 ブロックとスコープ
#[allow(dead_code)]
pub fn block_and_scope(){
    let mut total = 0;
    {
        for i in 1..10 {
            total += i;
        }
    }
    println!("total = {}", total);
}
