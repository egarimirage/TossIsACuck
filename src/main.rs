mod lua;




fn main() {
    println!("Hello, world!");

    lua::start_lua().expect("failed to launch lua!");

}