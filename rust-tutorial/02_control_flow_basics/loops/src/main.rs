fn while_expression() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("최종 x: {x}");
}


fn for_expression() {
    // 아래 표현식은 마지막 값을 포함하지 않음 e.g) python range(1, 5)
    // 만약 마지막 값을 포함하고 싶은 경우, 다음과 같이 작성 -> 1..=5
    for x in 1..5 {
        println!("x: {x}");
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }
}

// while (true)의 개념과 동일
fn loop_expression() {
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 5 {
            break;
        }
    }
}

// loops는 non-trivial 값을 반환하는 유일한 반복문
// do-while 개념으로 최소 한 번은 실행함
// 'outer: 레이블을 사용하면 바깥 루프까지 종료됨
fn loop_labels() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    // println이 아닌 print를 사용하는 경우, 개행이 없기 때문에, 다음 출력 내용과 겹칠 수 있음!
    // 만약 터미널이 ANSI escape 코드를 지원하는 환경이라면, % 같은 이상한 문자가 추가로 보일 수 있음.
    // e.g) elements searched: 6%
    print!("elements searched: {elements_searched}");
}

fn main() {
    while_expression();
    for_expression();
    loop_expression();
    loop_labels()
}