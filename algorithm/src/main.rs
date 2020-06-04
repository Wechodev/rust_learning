fn main(){
    let contents = "飞起来";
    some_process(contents);
    println!("{}",contents);
}
fn some_process(word: &str) {
    println!("some_process {}", word);
}