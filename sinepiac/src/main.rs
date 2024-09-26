use sinepiac::run;

pub fn main() {
    let status = run();
    std::process::exit(status as i32);
}
