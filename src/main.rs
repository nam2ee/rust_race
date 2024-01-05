fn main(){
    //step 1

    let s = String::from("hello");

    let len = calculate_length(&s);

    println!("The length of '{}' is {}.",s,len);

    // step2
    let mut s = String::from("hello");

    go(&mut s); // 참조자를 가변으로 선언하면 참조하고 있는 객체를 수정할 수 있다.

    // step3
    let mut s = String::from("hello");
    go(&mut s);

    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s; // read-only:okay

    let mut s = String::from("hello");
    let r1 = &s;
    //let r2 = &mut s; // race condition
    println!("{}",r1);
    println!("{}",r2); // r1이 이미 해제되었기 때문에 상관 없다!

    //Step4
    //let s: &String = dangle();
    let s: String = no_dangle();
    let s: &String = &no_dangle();
    

}

fn calculate_length(s: &String) -> usize{
    s.len()
}

fn go(str: &mut String){
    str.push_str(", world");
}
/*
fn dangle() -> &String{
    let s = String::from("dangle");
    &s //&s가 반환되지만, s는 이미 해제되고 없어서 문제가 발생한다.
}
*/

fn no_dangle() -> String{
    let s = String::from("no dangling");
    s// s가 그대로 반환된다
}

/*
crash example
fn main(){
    let s = String::from("hello");

    change(&s);
}
fn change(str: &String){
    str.push_str(", world");
} 
기본적으로 참조자도 불변성을 띄기 때문에, 참조하고 있는 객체를 수정할 수 없다.
 */