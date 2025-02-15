fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    
    // Rust는 암묵적 정수 변환을 허용하지 않음
    
    // 컴파일 에러 발생
    // takes_u32(y);

    // 명시적 타입 변환은 가능
    takes_u32(y as u32);
}