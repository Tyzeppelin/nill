

pub fn error(line: u32, message: &str) {
    report(line, "", message);
}

pub fn report(line: u32, wh_err: &str, message: &str) {
    println!("[line: {}] Error {} : {}", line, wh_err, message);
}
