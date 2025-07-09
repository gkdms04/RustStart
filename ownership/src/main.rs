// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str()이 문자열에 리터럴을 추가합니다

//     println!("{}", s); // 이 줄이 `hello, world!`를 출력합니다
    
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()은 String의 길이를 반환합니다

    (s, length)
}

// fn gives_ownership() -> String {             // gives_ownership은 자신의 반환 값을
//                                              // 자신의 호출자 함수로 이동시킬 것입니다
//     let some_string = String::from("yours"); // some_string이 스코프 안으로 들어옵니다
//     some_string                              // some_string이 반환되고 호출자 함수 쪽으로 이동합니다
// }

// // 이 함수는 String을 취하고 같은 것을 반환합니다
// fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로 들어옵니다
//     a_string  // a_string이 반환되고 호출자 함수 쪽으로 이동합니다
// }