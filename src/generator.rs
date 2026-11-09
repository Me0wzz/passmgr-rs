use rand::{Rng, rngs::OsRng};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
      abcdefghijklmnopqrstuvwxyz\
      0123456789!@#$%^&*";

pub fn generate_password(length: usize) -> String {
    let mut rng = OsRng;
    (0..length)
        .map(|_| {
            let index = rng.gen_range(0..CHARSET.len());
            CHARSET[index] as char
        })
        .collect()
}
