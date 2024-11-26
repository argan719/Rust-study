fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// if문에는 항상 boolean 타입의 조건식을 제공해야 한다. (number (X), number < 5 (O))



// 어떤 숫자가 0이 아닌 경우에만 if 코드 블록을 실행시키고자 할 때
fn main(){
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

// if-elseif 구문
fn main() {
    let number = 6;

    if number % 4 == 0{
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// let 구문에서 if 사용하기
fn main() {
    let condition = true;
    let number = if condition {5} else {6} ;

    println!("The value of number is: {number}");  
    // The value of number is: 5
}





// << 반복문 >>
// 러스트에는 loop, while, for 세 종류의 반복문 존재.

// << loop >>
// loop 키워드는 무한 루프. 
// lopp 키워드는 그만두라고 명시적으로 알려주기 전까지, 혹은 영원히 코드 블록을 반복 수행하도록 한다.
// loop 안에 break 키워드를 집어넣으면 루프를 멈출 수 있다.
fn main(){
    loop{
        println!("again!");
    }
}

// 해당 연산의 결과를 이후의 코드에 전달하고 싶은 경우. 
// 루프 정지를 위해 사용한 break 표현식 뒤에 반환하고자 하는 값을 넣으면 된다. 반복문 밖으로 반환되어 사용가능하게 된다.
fn main(){
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// << 루프 라벨 >> 로 여러 반복문 사이에 모호함 없애기
// 만일 루프 안에 루프가 있다면 break와 continue는 해당 지점의 바로 바깥쪽 루프에 적용된다.
// 루프에 루프 라벨(loop label)을 추가적으로 명시하면 break나 continue와 함께 이 키워드들이 바로 바깥쪽 루프 대신 라벨이 적힌 특정한 루프에 적용되도록 할 수 있다.
// 루프 라벨은 반드시 작은 따옴표로 시작해야 한다.
fn main(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;   
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");  // End count = 2
}
// 라벨이 명시되지 않은 첫 번째 break는 안쪽 루프만 탈출. 
// break 'counting_up;  구문은 바깥쪽 루프 탈출.


// << while문 >>
fn main(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

//  << for문 >>
fn main(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// for 반복문을 이용한 카운트다운 구현
// rev 메서드는 범위값을 역순으로 만들어줌.
fn main(){
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}