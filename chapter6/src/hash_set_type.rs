//!
//! 6章 ライブラリ型 サンプルプログラム
//!

use std::collections::HashSet;

/// ## 6-7.HashSet<T,S>
/// ### リスト6.30 インスタンスの生成
#[allow(dead_code)]
pub fn instantiate() {
    let set_a: HashSet::<i32> = HashSet::new();   // 整数HashSetを生成する
    // 容量が5の文字列HashSetを生成する
    let set_b: HashSet::<&str> = HashSet::with_capacity(5);
    // vec!マクロを利用してHashSetを生成する
    let set_c: HashSet<i32> = vec![10, 20, 30].into_iter().collect();
    println!("len() = {}, set_a = {:?}", &set_a.len(), &set_a);
    println!("len() = {}, set_b = {:?}", &set_b.len(), &set_b);
    println!("len() = {}, set_c = {:?}", &set_c.len(), &set_c);
}