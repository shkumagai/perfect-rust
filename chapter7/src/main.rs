//!
//! 7章 関数
//!

mod basic_function;
mod function_type;
mod generics;
mod ownership;
mod reference;
mod life_time;
mod option_type;
mod result_type;
mod closure;

///
/// メイン関数
///
fn main() {
    // basic_function::print_message_1();   // 基本的な関数の定義
    // basic_function::print_message_2(String::from("引数付き関数"));   // 引数付き関数の定義
    // let mut message = String::from("7-1-3 ");
    // basic_function::print_message_3(&mut message);
    // println!("関数利用後の文字列 = {:?}", &message);
    // println!("{:?}", basic_function::print_message_4(String::from("")));
    // println!("{:?}", basic_function::print_message_4(String::from("戻り値付き関数")));

    // function_type::use_function_1();    // 関数を変数に代入して利用する
    // function_type::use_function_2();    // typeキーワードを利用する
    // function_type::use_function_3();    // 関数型を返す
    // generics::use_add();                // ジェネリクス型を利用した関数の実行
    // generics::use_sub();                // ジェネリクス型を利用した関数の実行
    // ownership::ownership_1();           // 代入による所有権の移動
    // ownership::ownership_2();           // 代入による所有権の移動
    // ownership::ownership_3();           // 代入による所有権の移動
    // ownership::ownership_4();           // 引数渡しによる所有権の移動
    // ownership::ownership_5();           // 引数渡しによる所有権の移動
    // ownership::ownership_6();           // リターンによる所有権の移動
    // ownership::ownership_7();           // スコープ

    // reference::reference_1();           // 参照の取得
    // reference::reference_2();           // 参照の取得
    // reference::reference_3();           // ミュータブルな参照は1つだけ
    // reference::reference_4();           // 参照の混在はできない

    // life_time::life_time_1();           // ライフタイムの基本
    // life_time::life_time_2();           // 参照を別の変数へ代入する
    // life_time::life_time_3();           // 値の参照を返す関数
    // life_time::life_time_4();           // 引数の参照を返す関数
    // life_time::life_time_5();           // 関数実行にも注意

    // let mut x = Vec::<i32>::new();
    // life_time::push(&mut x);
    // println!("{:?}", x);

    // option_type::declare();             // 変数宣言
    // option_type::use_div();             // 関数での利用
    // option_type::method_1();            // 値の評価メソッド
    // option_type::method_2();            // 値の取得メソッド
    // option_type::method_3();            // コンビネータ
    // option_type::method_4();            // Result型に変換するメソッド
    // let r = match option_type::method_5() { // ?演算子の利用
    //     Some(r) => r,
    //     None => "計算できません。".to_owned()
    // };
    // println!("{}", r);

    // result_type::value_setting();       // 変数宣言
    // result_type::use_div();             // 関数からの利用
    // result_type::method_1();            // 値の検証メソッド
    // result_type::method_2();            // 値の取得メソッド
    // result_type::method_3();            // コンビネータ
    // result_type::method_4();            // Option型へ変換する
    // let r = match result_type::method_5() { // ?演算子の利用
    //     Ok(r) => r,
    //     Err(error) => error
    // };
    // println!("{}", r);

    // closure::closure_sum();             // 基本的なクロージャ
    // closure::move_1();                  // moveによる所有権の移動
    // closure::move_2();                  // moveによる所有権の移動
    // closure::use_impl_where_1();        // 合計を求めるクロージャ
    closure::use_impl_where_2();        // 合計を求めるクロージャ
}
