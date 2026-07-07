use std::str::FromStr;

use argon2::{Algorithm, Argon2, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;

use crate::error::VerifyError;

/// 密码工具类
/// 使用 argon2库作密码的加解密工具
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordUtils {
    pub password_hash: String,
    pub salt: String,
}

impl PasswordUtils {
    /// 明文密码进行加密并返回hash密码和salt值
    pub fn encrypt(password: impl AsRef<[u8]>) -> Self {
        let encrypt = Argon2::default();
        let salt_string = SaltString::generate(OsRng);
        let hash_password = encrypt.hash_password(password.as_ref(), &salt_string)
            .expect("密码加密失败 — 盐值生成不应失败");
        let password_hash = hash_password.hash.unwrap().to_string();
        let salt = salt_string.to_string();
        Self { password_hash, salt }
    }

    /// 通过输入明文+密码盐的方式进行密码验证
    /// 返回 Ok(()) 表示密码正确，Err(VerifyError) 表示密码错误或解析失败
    pub fn verify(password: impl AsRef<[u8]>, pass: &str, salt: &str) -> Result<(), VerifyError> {
        let salt_string = match SaltString::from_b64(salt) {
            Ok(s) => s,
            Err(_) => return Err(VerifyError::PasswordInvalid),
        };
        let pass = match argon2::password_hash::Output::from_str(pass) {
            Ok(p) => p,
            Err(_) => return Err(VerifyError::PasswordInvalid),
        };
        let data = PasswordHash {
            algorithm: Algorithm::default().into(),
            version: Some(Version::default().into()),
            params: Default::default(),
            salt: Some(salt_string.as_salt()),
            hash: Some(pass),
        }
            .to_string();
        let hash = match PasswordHash::new(data.as_str()) {
            Ok(h) => h,
            Err(_) => return Err(VerifyError::PasswordInvalid),
        };
        let args: &[&dyn PasswordVerifier] = &[&Argon2::default()];

        hash.verify_password(args, password).map_err(|_| VerifyError::PasswordInvalid)
    }
}
