// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz"); // 将 string2 的定义移到内部作用域之外
    {
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    } // 内部作用域结束，但是 result 已经不再被使用，所以不会有问题
}
