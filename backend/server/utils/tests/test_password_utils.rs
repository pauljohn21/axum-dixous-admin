use utils::prelude::PasswordUtils;

const PASSWORD: &str = "123456";

#[test]
fn test_encrypt() {
    let hash = PasswordUtils::encrypt(PASSWORD);
    println!("密码{:?}", hash.password_hash);
    println!("加密{:?}", hash.salt);
}

#[test]
fn test_verify() {
    let p = "W62FiV1GbNz2FS+LhWgQxw0BTtS/gNDo7joXtorYOsc";
    let s = "qdUA6i0Bh1gNDc/7lreyTA";
    let v = PasswordUtils::verify(PASSWORD, p, s);
    println!("{:?}", v)
}
