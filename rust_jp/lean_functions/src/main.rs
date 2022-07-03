fn main() {
    let y = {
        let x = 3;
        // 行末に;をつけるとstatementとして評価されるため、コンパイルエラーになる
        x + 1
    };

    println!("value is {}", y);
}
