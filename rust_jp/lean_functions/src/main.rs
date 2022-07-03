fn main() {
    let y = {
        let x = 3;
        // 行末に;をつけるとstatementとして評価されるため、コンパイルエラーになる
        x + 1
    };

    let number = if y == 4 {
        "4です"
    } else {
        "４ではありません"
    };

    println!("value is {}", number);
}
