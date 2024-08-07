fn main() {
    //---------------------------if 
    // let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else { 
    //     println!("condition was false");
    // }

    // let number = 3;

    // if number {
    //     println!{"number was three"};    //오류 다른 언어와 다르게 숫자면 아예 안받는다
    // }

    // if number != 0 {    //0이 아닌지 판단해서 bool값을 내놓음
    //     println!{"number was something other than zero"};    
    // }

    //---------------------------else if
    // let number = 6;

    // if number % 4 == 0 {   
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {    // 이 경우 여기서 조건문이 발동함으로 뒤의 else if, else를 무시하고 해당 이벤트를 발생시키고 넘어간다.
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // } // 각각 나눈 나머지가 0이면 조건문 발동

    //---------------------------Using if in a let Statement

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!{"The value of number is: {number}"};
    // let condition = true;
    // let number = if condition { 5 } else { "six" };

    // println!{"The value of number is: {number}"};

}