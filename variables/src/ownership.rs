
// << 소유권과 함수 >>
fn main() {
    let s = String::from("hello");  // s가 스코프 안으로 들어옴

    takes_ownership(s);    // s의 값이 함수로 이동됩니다..
                           // 따라서 여기서는 더 이상 s 가 유효하지 않다.
    
    let x = 5;             // x가 스코프 안으로 들어옴

    makes_copy(x);         // x가 함수로 이동될 것입니다만, i32는 Copy이므로 앞으로 계속 x를
                           // 사용해도 좋습니다.
}  // 여기서 x가 스코프 밖으로 벗어나고 s도 그렇게 됩니다. 그러나 s의 값이 이동되었으므로 별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String) {  // some_string이 스코프 안으로 들어옵니다.
    println!("{}", some_string);
}  // 여기서 some_string이 스코프 밖으로 벗어나고 drop이 호출된다.
   // 메모리가 해제된다.

fn makes_copy(some_integer: i32) { // some_integer가 스코프 안으로 들어옵니다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어난다. 별다른 일이 발생하지 않는다.

// 러스트는 takes_ownership 함수를 호출한 이후에 s를 사용하려 할 경우, 컴파일 에러 발생시킴.



 // << 반환값과 스코프 >>
 fn main(){
    let s1 = gives_ownership();     // gives_ownership이 자신의 반환값을 s1로 이동시킨다.

    let s2 = String::from("hello"); // s2가 스코프 안으로 들어옵니다.

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back로 이동되는데, 이 함수 또한 자신의 반환 값을 
                                        // s3로 이동시킨다.
 } // 여기서 s3가 스코프 밖으로 벗어나면서 버려집니다. s2는 이동되어서 아무 일도 일어나지 않습니다.
   // s1은 스코프 밖으로 벗어나고 버려진다.

   fn gives_ownership() -> String {           // gives_ownership은 자신의 반환 값을 자신의 호출자 함수로 이동시킬 것입니다.

    let some_string = String::from("yours");  // some_string이 스코프 안으로 들어옵니다.

    some_string                               // some_string이 반환되고
                                              // 호출자 함수 쪽으로 이동합니다.                    
   }

// 이 함수는 String을 취하고 같은 것을 반환합니다
fn takes_and_gives_back(a_string: String) -> String {  // a_string이 스코프 안으로 들어옵니다

    a_string  // a_string이 반환되고 호출자 함수 쪽으로 이동합니다
}

// 튜플을 사용하여 여러 값을 반환하는 것이 가능하다.
fn main(){
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();  // len은 String의 길이를 반환

    (s, length)
}


// 다행히도, 러스트에는 소유권 이동 없이 값을 사용할 수 있는 참조자 (reference) 라는 기능을 가지고 있다.!