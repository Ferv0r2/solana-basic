// 시간 복잡도: O(n^2)
// 반복문이나 DP로 접근하면 O(n)까지 빠르게 처리 가능할 것으로 보임
// 다음 섹션도 열심히 배우자 :)

fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    }
    return fib(n-1) + fib(n-2);
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}