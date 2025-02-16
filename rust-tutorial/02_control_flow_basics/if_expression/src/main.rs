fn main() {
    let x = 10;
    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("100 미만");
    } else {
        println!("100 이상");
    }

    // 한 줄 if문
    let size = if x < 20 {"20 미만"} else {"20 이상"};
    println!("숫자 크기: {}", size);
}