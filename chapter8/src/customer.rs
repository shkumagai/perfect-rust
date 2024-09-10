///
/// 8章 構造体のサンプルコード
///


/// ## 顧客を表す構造体
/// ## 8-3.型関連定数と型関連関数
/// ### リスト8.6 型関連定数
#[allow(dead_code)]
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
