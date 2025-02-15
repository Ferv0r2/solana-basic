// 불변 문자열: &str
// 가변 문자열: String
// 문자열 추가 방법: push_str()

fn main() {
    let greeting: &str = "  인사말";
    let planet: &str = "🪐";
    let mut sentense = String::new();
    sentense.push_str(greeting);
    sentense.push_str(", ");
    sentense.push_str(planet);
    println!("Last sentense: {}", sentense);
    println!("{:?}", &sentense[0..5]);
    // 실행 시, 에러 발생
    // 이모지는 4바이트, 중간 바이트에 접근 시 잘못된 바이트 범위 에러 발생
    // println!("{:?}", &sentense[12..13]);
}
