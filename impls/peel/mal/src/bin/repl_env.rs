use std::collections::BTreeMap;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}

fn div(x: i32, y: i32) -> i32 {
    x / y
}

pub fn get_repl_env() -> BTreeMap<&'static str, fn(i32, i32) -> i32> {

    let env_add: fn(i32, i32) -> i32 = add;
    let env_sub: fn(i32, i32) -> i32 = sub;
    let env_mul: fn(i32, i32) -> i32 = mul;
    let env_div: fn(i32, i32) -> i32 = div;

    let mut repl_env = BTreeMap::new();
    repl_env.insert("+", env_add);
    repl_env.insert("-", env_sub);
    repl_env.insert("*", env_mul);
    repl_env.insert("/", env_div);

    return repl_env;
}

fn main() {
    let a = get_repl_env();
    println!("{}", a["+"](5, 7));
}
