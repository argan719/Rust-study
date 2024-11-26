
// fn: 새로운 함수 선언
// 함수 선언 예약어: fn
// rust 에서는 함수나 변수 이름을 작성할 때, 모든 글자는 소문자로 쓰고 _로 단어를 구분하는 방식을 사용한다. 

// 함수 정의 예제
fn main(){
    println!("Hello, World!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}



// 매개변수 추가
fn main(){
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}


// 매개변수 여러 개인 경우
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");  // The measurement is: 5h
}





// << 구문과 표현식 >>
// 러스트는 표현식 기반의 언어.

// 구문: 어떤 동작을 수행하고 값을 반환하지 않는 명령.
// 표현식: 결괏값을 평가한다. 

// 구문 -> let키워드로 변수 선언하고 값 할당, 함수 정의, (그냥 값을 반환하지 않는 것들이라고 이해하면 됨)

// 러스트 코드의 대부분은 표현식이다. 표현식은 종결을 나타내는 세미콜론을 쓰지 않는다. 
// 만약, 표현식 끝에 세미콜론을 추가하면, 표현식은 구문으로 변경되고 값을 반환하지 않게 된다.

fn main(){
    let y = {
        let x = 3;
        x + 1      // 세미콜론으로 안 끝남.
    };

    println!("The value of y is: {y}");
}






// << 반환 값을 갖는 함수 >>
// 함수는 값을 반환할 수 있는데 반환되는 값을 명명해야 할 필요는 없지만, 그 값의 타입은 화살표(->) 뒤에 선언되어야 한다.
// -> (반환값의)자료형
// return으로 일찍 값을 반환할 수 있지만, 대부분의 함수들은 암묵적으로 마지막 표현식 값을 반환한다.

fn five() -> i32 {
    5     // 5는 five 함수의 반환 값. (5라서 반환 타입 i32로 선언). / 반환하려는 값에 대한 표현식이기 때문에 세미콜론이 없다. 즉, 함수 내에 5라는 expression이 존재한다. 따라서 반환값은 5이다.
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");  // The value of x is: 5
}
// let x = five(); => 함수의 반환 값을 변수의 초깃값으로 사용





fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");  // The value of x is: 6
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
// x + 1 끝에 세미콜론이 추가되어 표현식이 구문으로 변경되면 에러가 발생함.
// plus_one 함수 정의할 때 i32 값을 반환한다고 해놨는데 ( -> i32) 
// 구문으로 변경되어 아무것도 반환되지 않아 함수가 정의된 내용과 상충하게 되어 에러




// Expression(표현식): 실행 시 값이 반환됨
// Statement(구문): 실행 시 값이 반환되지 않고 특정 처리를 해줌.
