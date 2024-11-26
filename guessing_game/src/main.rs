
use std::io;  
use rand::Rng;  // Rng는 난수 생성기를 구현한 메서드들을 정의한 trait
use std::cmp::Ordering;  // Less, Greater, Equal

fn main() {
    println!("Guess the number!");
    // 임의의 숫자 생성 
    // i32 32비트 정수, u32는 32비트의 부호 없는 정수, i64는 64비트의 정수. default는 i32
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // rand::thread_rng 난수 생성기 제공
    // gen_range 함수 호출. Rng trait에 정의되어 있음.
    // gen_range 함수는 범위 표현식을 인수로 받아서 해당 범위 내 임의의 숫자 생성.
    // 범위 표현식: start..=end (상한선, 하한선 포함)  ex) 1부터 100 사이면 1..=100 으로 지정하면 됨.

    // println!("The secret number is: {secret_number}");


    loop{  // 무한루프 제공. 
        println!("Please input your guess.");

        let mut guess = String::new();   // 사용자 입력값 받을 변수 생성: guess라는 이름의 가변 변수.  new로 비어있는 새 문자열을 생성한다.
        /* let apples = 5; */
        // 러스트에서 변수는 기본적으로 불변인데, (상수로 선언한다는 거임)
        // 변수를 가변, 변경 가능하도록 선언하려면 변수명 앞에 mut을 추가하면 된다.
    
        /* 
           let apples = 5;  // immutable
           let mut bananas = 5;  // mutable 
        */
    
        // std::io::stdin 생략가능
        io::stdin()
            .read_line(&mut guess)  // 사용자로부터 입력받기 위해 read_line 함수 호출. guess 변수에 입력 받겠다! 이때, guess 변수는 가변이어야 한다.
            .expect("Failed to read line");  // io::stdin().read_line(&nut guess).expect("Failed to read line");
    
        // String을 정수형으로 변환.
        // 섀도잉(shadowing) : guess를 중복해서 선언함. guess라는 변수명을 재사용할 수 있게 허용해주는 것.
        // 어떤 한 타입의 값을 다른 타입으로 바꾸고 싶을 때 그냥 재선언을 통해 바꿔줄 수 있다. (shadowing)
     // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // String인스턴스의 trim 메서드: 처음,끝 공백문자 제거
        // parse(): 문자열을 다른 타입으로 바꿔줌. let guess: u32를 사용하여 필요로 하는 숫자 타입 입력. 
    

        // 사용자가 숫자가 아닌 값을 입력할 때 프로그램을 종료시키는 대신 이를 무시하고 계속 입력할 수 있도록 만들기 위해 아래와 같이 변환문 작성
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        // expect메서드 호출 대신 match 표현식으로 바꾸어 에러 발생 시 즉시 종료가 아닌 에러를 처리하도록 바꿈.
        // !! parse 메서드가 Result 타입을 반환, Result는 Ok나 Err variant를 가진 열거형임을 기억.!!
        // Err(_) 에서 _은 'catchall' 값. 모든 값에 매칭될 수 있음. Err내에 무슨 값이 반환됐던지 매칭되도록.
        // 즉, Error가 발생하면 그게 무슨 에러든 continue 하도록.


        println!("You guessed: {guess}"); // {변수명}. format처럼 사용. {출력하고 싶은 것}
    
        match guess.cmp(&secret_number){  // cmp() : 두 값을 비교(guess와 secret_number 비교). guess는 String이고 secret_number는 정수형이기 때문에 에러 발생. 타입을 맞춰줘야 한다.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),    // 50 > 38
            Ordering::Equal => {
                println!("You win!");
                break;
            } 
        }
    }

}


// &는 reference. rust는 참조자 사용 가능.
// 참조자는 변수처럼 기본적으로 불변이다. 따라서 &mut guess 로 작성하여 가변으로 만듦.

// read_line은 실행 후 리턴값이 있는데 
// expect 함수를 통해 이를 확인할 수 있도록 한다.


// 변수 2개를 한꺼번에 출력
// let x = 5;
// let y = 10;
// println!("x={x} and y + 2 = {}", y+2);
// 실행결과: x = 5 and y + 2 = 12

