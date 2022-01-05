fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn foo(bar: Option<u8>) -> Option<u8> {
    if let Some(val) = bar {
        Some(val)
    } else {
        None
    }
}
