//!
//! 4章 基本データ型
//! サンプルプログラム
//!

/// ### 4-2.浮動小数点型
/// #### リスト4.5 浮動小数点リテラル
#[allow(dead_code)]
pub fn floating_literal() {
    println!("a = {}", 10.5f32);    // f32型
    println!("b = {}", -10.5f64);   // f64型
    println!("c = {}", 10.5_f32);   // f32型
    println!("d = {}", -10.5_f64);  // f64型
}

/// ### 4-2.浮動小数点型
/// #### リスト4.6 浮動小数点型の定数
#[allow(dead_code)]
pub fn floating_constant() {
    println!("RADIX = {}", f32::RADIX);                     // 指数表現の基数
    println!("MANTISSA_DIGITS = {}", f32::MANTISSA_DIGITS); // RADIX進法での有効桁数
    println!("DIGITS = {}", f32::DIGITS);                   // 最大の10進桁数
    println!("EPSILON = {}", f32::EPSILON);                 // マシンイプシロン値
    println!("MIN = {}", f32::MIN);                         // 最小値
    println!("MIN_POSITIVE = {}", f32::MIN_POSITIVE);       // 正の最小正規値
    println!("MIN_EXP = {}", f32::MIN_EXP);                 // 最小のRADIX進の指数
    println!("MIN_10_EXP = {}", f32::MIN_10_EXP);           // 最小の10進の指数
    println!("MAX = {}", f32::MAX);                         // 最大値
    println!("MAX_EXP = {}", f32::MAX_EXP);                 // 最大のRADIX進の指数
    println!("MAX_10_EXP = {}", f32::MAX_10_EXP);           // 最大の10進の指数
    println!("NAN = {}", f32::NAN);                         // 非数
    println!("INFINITY = {}", f32::INFINITY);               // 正の無限大
    println!("NEG_INFINITY = {}", f32::NEG_INFINITY);       // 負の無限大
}

/// ### 4-2.浮動小数点型
/// #### リスト4.7 浮動小数点型の主なメソッド
#[allow(dead_code)]
pub fn methods() {
    let x = 10.5_f64;
    println!("cbrt() = {}", x.cbrt());      // 立方根を計算する
    println!("ceil() = {}", x.ceil());      // 小数点以下を切り上げする
    println!("floor() = {}", x.floor());    // 小数点を丸める
}
