///
/// 8章 構造体のサンプルコード
///

/// ## 顧客を表す構造体
/// ## 8-3.型関連定数と型関連関数
/// ### リスト8.6 型関連定数
#[allow(dead_code)]
#[derive(Debug,Clone)]  // トレイトを追加
struct Customer {
    id: u32,            // 顧客番号を表すフィールド
    name: String,       // 氏名を表すフィールド
    address: String,    // 住所を表すフィールド
    email: String       // メールアドレスを表すフィールド
}
impl Customer { // Customer構造体の型関連定数
    // フィールドの最小値と最大値を表す定数
    const ID_MIN:u32 = 1;
    const ID_MAX:u32 = 10000;

    /// ## 8-3.型関連定数と型関連関数
    /// ### リスト8.8 インスタンスを生成する型関連関数
    /// ### 引数 顧客番号:id, 氏名:name, 住所:address, メールアドレス:email
    fn new(id: u32, name: String, address: String, email: String) -> Self {
        Self {id, name, address, email}
    }

    /// ## 8-4.メソッド
    /// ### リスト8.9 nameフィールドの値を返すメソッド
    /// ### 引数 なし
    /// ### 戻り値 　nameフィールドの値（クローン）
    #[allow(dead_code)]
    fn get_name(&self) -> String {
        self.name.clone()
    }

    /// ## 8-4.メソッド
    /// ### リスト8.9 nameフィールドの値を変更するメソッド
    /// ### 引数 nameフィールドの値
    /// ### 戻り値 　なし
    #[allow(dead_code)]
    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

/// ## 8-3.型関連定数と型関連関数
/// ### リスト8.6 型関連定数
#[allow(dead_code)]
pub fn use_constant() {
    println!("ID_MIN = {}", Customer::ID_MIN);
    println!("ID_MAX = {}", Customer::ID_MAX);
}

/// ## 8-3.型関連定数と型関連関数
/// ### リスト8.7 インスタンスを生成する型関連関数
#[allow(dead_code)]
pub fn use_new() {
    // インスタンスを生成する型関連関数の利用
    let customer = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@sample.com")
    );
    // フィールドの内容を出力する
    println!("id = {}", customer.id);
    println!("name = {}", customer.name);
    println!("address = {}", customer.address);
    println!("email = {}", customer.email);
}

/// ## 8-4.メソッド
/// ### リスト8.9 メソッドの実装と利用
#[allow(dead_code)]
pub fn use_method() {
    // インスタンスの生成
    let mut customer = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@sample.com")
    );
    customer.set_name(String::from("鈴木花子"));    // メソッドでnameフィールドの値を変更する
    println!("{}", customer.get_name());    // メソッドでフィールドの値を取得する
}

/// ## 8-5.ユーティリティトレイト
/// ### リスト8.11 Debugトレイト
#[allow(dead_code)]
pub fn use_debug(){
    // インスタンスの生成する
    let customer = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@sample.com")
    );
    println!("{:?}" , customer); // インスタンスの内容を確認する
}

/// ## 8-5.ユーティリティトレイト
/// ### リスト8.12 Cloneトレイト
#[allow(dead_code)]
pub fn use_clone(){
    // インスタンスの生成する
    let customer = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@sample.com")
    );
    println!("customerの複製 = {:?}" , customer.clone()); // インスタンスの複製結果を出力する
}

/// ## 8-5.ユーティリティトレイト
/// ### リスト8.13 Dropトレイト
impl Drop for Customer { // Dropトレイトの実装ブロック
    fn drop(&mut self) {
        println!("{}のインスタンスを破棄します。", self.name);
    }
}

#[allow(dead_code)]
pub fn use_drop() {
    // インスタンスを生成する
    let customer_1 = Customer::new(
        100,
        String::from("山田太郎"),
        String::from("東京都新宿区"),
        String::from("yamada@sample.com"),
    );
    let mut customer_2 = customer_1.clone();
    customer_2.set_name(String::from("田中次郎"));
}

/// ## 8-5.ユーティリティトレイト
/// ### リスト8.14 Defaultトレイト
impl Default for Customer { // Defaultトレイトの実装ブロック
    fn default() -> Self {
        Self { id: 0, name: String::from(""), address: String::from(""), email: String::from("") }
    }
}

#[allow(dead_code)]
pub fn use_default() {
    let customer = Customer::default(); // インスタンスを生成する
    println!("default() = {:?}", customer);
}

// /// ## 8-5.ユーティリティトレイト
// /// ### リスト8.15 Fromトレイト
// impl From<&Vec<&str>> for Customer {    // Fromトレイトの実装ブロック
//     /// ### 引数 文字列スライスベクタの参照
//     fn from(value: &Vec<&str>) -> Self {
//         Self {
//             id: value[0].parse::<u32>().unwrap(),
//             name: value[1].to_owned(),
//             address: value[2].to_owned(),
//             email: value[3].to_owned(),
//         }
//     }
// }

// #[allow(dead_code)]
// pub fn use_from() {
//     // 文字列スライスのベクタを生成
//     let value = vec!["100", "山田太郎", "東京都新宿区", "yamada@sample.com"];
//     let customer = Customer::from(&value);  // ベクタからCustomerの変換結果を得る
//     println!("from() = {:?}", customer)
// }

/// ## 8-5.ユーティリティトレイト
/// ### 8.16 リストTryFromトレイト
impl TryFrom<&Vec<&str>> for Customer { // TryFromトレイトの実装ブロック
    type Error = String;    // エラーの場合の型を指定する
    /// ### 引数 文字列スライスベクタの参照
    fn try_from(value: &Vec<&str>) -> Result<Self, Self::Error> {
        // 文字列を整数に変換する
        let new_id = match value[0].parse::<u32>() {
            Ok(value) => value,
            Err(error) => return Err(error.to_string()) // エラーならメッセージを返す
        };
        // インスタンスを生成
        Ok(Self {
            id: new_id,
            name: value[1].to_owned(),
            address: value[2].to_owned(),
            email: value[3].to_owned(),
        })
    }
}

#[allow(dead_code)]
pub fn use_try_from() {
    //文字列スライスのベクタを生成
    let value = vec!["ABC", "山田太郎", "東京都新宿区", "yamada@sample.com"];
    let customer = Customer::try_from(&value);
    if customer.is_ok() {
        println!("try_from() = {:?}", customer.unwrap());
    } else {
        println!("try_from() = {:?}", customer.unwrap_err());
    }
}
