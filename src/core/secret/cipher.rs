use anyhow::Result;
use std::io::{Read, Write};

pub fn encrypt(data: &[u8], recipient: &age::x25519::Recipient) -> Result<Vec<u8>> {
    let encryptor = age::Encryptor::with_recipients(vec![Box::new(recipient.clone())])
        .expect("we provided a recipient");

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(data)?;
    writer.finish()?;

    Ok(encrypted)
}

pub fn decrypt(encrypted_data: &[u8], identity: &age::x25519::Identity) -> Result<Vec<u8>> {
    let decryptor = match age::Decryptor::new(encrypted_data)? {
        age::Decryptor::Recipients(d) => d,
        _ => anyhow::bail!("Unsupported encrypted format"),
    };

    let mut decrypted = vec![];
    let mut reader = decryptor.decrypt(std::iter::once(identity as &dyn age::Identity))?;
    reader.read_to_end(&mut decrypted)?;

    Ok(decrypted)
}
