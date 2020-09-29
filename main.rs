

// # hello Rust
// * test1
// * test2
fn main() {
    // !はrustマクロというらしい
    println!("Hello world!");

    //変数宣言
    let a = 1;

    //型宣言も可能
    let b :i32 = 2;

    //デフォルトimmutableだけど、mutでmutableになる    
    let mut c = 3;

    // 複数宣言する場合は()をつける
    let (d, e, f) = (4, 5, 6);

}
