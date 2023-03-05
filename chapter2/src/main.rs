//!
//! ## 2章 演算子
//!
mod arithmetic;
mod assign;
fn main(){
    // arithmetic::symbol();                       // 算術演算子の利用と結果の出力
    // arithmetic::methods(10, 6);                 // 算術演算メソッドの利用と結果の出力
    // arithmetic::overflow();                     // オーバーフローを起こす計算
    // arithmetic::ignore_overflow();              // オーバーフローを無視する
    // arithmetic::check_option_overflow();        // Option<T>型でオーバーフローを確認する
    // arithmetic::check_boolean_overflow();       // 論理地でオーバーフローを確認する
    // arithmetic::return_max_overflow();          // オーバーフローしたら最大値を返す

    // assign::assign_value(10, 12.567);           // 代入演算子の利用
    assign::compound_assign(10, 6);             // 複合代入演算子の利用
}
