// Rust는 메서드에서 마지막 세미콜론이 포함된 값을 return함
// 물론, 가능하다는 것이지 실무 레벨에서는 세미 콜론을 꼼꼼히 넣어주거나 코드 순서를 잘 맞춰주는 것이 좋아 보임
fn blocks() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}

// 쉐도잉은 불변성을 유지하면서 내부 스코프의 상태나 값을 변경하고 이를 할당할 때 사용하는 것으로 보임
// mut은 같은 메모리를 사용하며 가변성을 허용하지만
// 쉐도잉은 완전히 새로운 변수를 덮어쓰면서 메모리가 변경되고 기존 변수의 불변성을 유지함
// 보통 unwrap()에서 유용하게 사용한다고 함.
fn scopes() {
    let a = 10;
    println!("이전: {a}");
    {
        let a = "hello";
        println!("내부 범위: {a}");

        let a = true;
        println!("내부 범위 섀도 처리됨: {a}");
    }

    println!("이후: {a}");
}

fn scopes_unwrap() {
    let input = "42";
    let input: i32 = input.parse().unwrap(); // 기존 변수명 유지 가능
    println!("{input}");
}

fn main() {
    blocks();
    scopes();
    scopes_unwrap();
}