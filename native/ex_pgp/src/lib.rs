use pgp::composed::signed_key::*;
use pgp::composed::{
    key::SecretKeyParamsBuilder, key::SubkeyParamsBuilder, Deserializable, KeyType, Message,
};
use pgp::crypto::{hash::HashAlgorithm, sym::SymmetricKeyAlgorithm};
use pgp::ser::Serialize;
use pgp::types::{CompressionAlgorithm, KeyTrait, PublicParams, SecretKeyTrait};
use rsa::{pkcs8::EncodePublicKey, BigUint, RsaPublicKey};
use rustler::{Atom, Binary, Encoder, Env, NifResult, OwnedBinary, Term};
use smallvec::*;
use std::io;

mod atoms {
    rustler::atoms! {
        ok,
        error,

        invalid_data,
        invalid_key,
        missing_signature,
        invalid_signature,
        decryption_failed,
    }
}

#[rustler::nif]
fn generate<'a>(env: Env<'a>, key_size: u32) -> NifResult<Term<'a>> {
    let mut subkey_params_builder = SubkeyParamsBuilder::default();
    subkey_params_builder
        .key_type(KeyType::Rsa(key_size))
        .can_encrypt(true);
    let subkey_params = subkey_params_builder
        .build()
        .expect("Must be able to create subkey params");

    let mut key_params = SecretKeyParamsBuilder::default();
    key_params
        .key_type(KeyType::Rsa(key_size))
        .can_create_certificates(false)
        .can_sign(true)
        .can_create_certificates(true)
        .primary_user_id("JumpWire".into())
        .preferred_symmetric_algorithms(smallvec![
            SymmetricKeyAlgorithm::AES256,
            SymmetricKeyAlgorithm::AES192,
            SymmetricKeyAlgorithm::AES128,
            SymmetricKeyAlgorithm::CAST5,
        ])
        .preferred_hash_algorithms(smallvec![
            HashAlgorithm::SHA2_512,
            HashAlgorithm::SHA2_384,
            HashAlgorithm::SHA2_256,
            HashAlgorithm::SHA2_224,
        ])
        .preferred_compression_algorithms(smallvec![
            CompressionAlgorithm::ZLIB,
            CompressionAlgorithm::BZip2,
            CompressionAlgorithm::ZIP,
            CompressionAlgorithm::Uncompressed,
        ])
        .subkeys(vec![subkey_params]);
    let secret_key_params = key_params
        .build()
        .expect("Must be able to create secret key params");
    let secret_key = secret_key_params
        .generate()
        .expect("Failed to generate a plain key.");
    let passwd_fn = || String::new();
    let signed_secret_key = secret_key
        .sign(passwd_fn)
        .expect("Must be able to sign its own metadata");
    let public_key = signed_secret_key.public_key();
    let signed_public_key = public_key
        .sign(&signed_secret_key, passwd_fn)
        .expect("Must be able to sign its own metadata");

    let serialized_secret = signed_secret_key.to_armored_string(None).unwrap();
    let serialized_public = signed_public_key.to_armored_string(None).unwrap();

    Ok((atoms::ok(), serialized_secret, serialized_public).encode(env))
}

#[rustler::nif]
fn created_at<'a>(env: Env<'a>, pem: Binary) -> Result<Term<'a>, Atom> {
    let cursor = io::Cursor::new(pem.as_slice());
    let (secret_key, _) = match SignedSecretKey::from_armor_single(cursor) {
        Ok(v) => v,
        _ => return Err(atoms::invalid_key()),
    };

    let timestamp = secret_key.primary_key.created_at().timestamp();

    Ok(timestamp.encode(env))
}

#[rustler::nif]
fn export_public_key<'a>(env: Env<'a>, pem: Binary) -> Result<Term<'a>, Atom> {
    let cursor = io::Cursor::new(pem.as_slice());
    let (secret_key, _) = match SignedSecretKey::from_armor_single(cursor) {
        Ok(v) => v,
        _ => return Err(atoms::invalid_key()),
    };
    let public_key = secret_key.public_key();
    let signed_public_key = public_key
        .sign(&secret_key, || String::new())
        .expect("Must be able to sign its own metadata");

    let serialized_public = signed_public_key.to_armored_string(None).unwrap();
    Ok(serialized_public.encode(env))
}

#[rustler::nif]
fn export_rsa_pubkey<'a>(env: Env<'a>, pem: Binary) -> Result<Term<'a>, Atom> {
    let cursor = io::Cursor::new(pem.as_slice());
    let (public_key, _) = match SignedPublicKey::from_armor_single(cursor) {
        Ok(v) => v,
        _ => return Err(atoms::invalid_key()),
    };

    let serialized_public = match public_key.primary_key.public_params() {
        PublicParams::RSA { ref n, ref e } => {
            let rsa_pubkey =
                RsaPublicKey::new(BigUint::from_bytes_be(n), BigUint::from_bytes_be(e));
            let key = match rsa_pubkey {
                Ok(key) => key,
                _ => return Err(atoms::invalid_key()),
            };

            match key.to_public_key_pem(rsa::pkcs8::LineEnding::LF) {
                Ok(pem) => pem,
                _ => return Err(atoms::invalid_key()),
            }
        }
        _ => return Err(atoms::invalid_key()),
    };

    Ok(serialized_public.encode(env))
}

#[rustler::nif]
fn encrypt<'a>(env: Env<'a>, pem: Binary, data: Binary) -> NifResult<Term<'a>> {
    let cursor = io::Cursor::new(pem.as_slice());
    let (key, _) = SignedPublicKey::from_armor_single(cursor).unwrap();
    let mut rng = rand::thread_rng();

    let msg = Message::new_literal_bytes("", data.as_slice())
        .compress(CompressionAlgorithm::ZLIB)
        .and_then(|msg| msg.encrypt_to_keys(&mut rng, SymmetricKeyAlgorithm::AES256, &[&key]))
        .unwrap();

    let ciphertext = msg.to_bytes().unwrap();
    let mut bin = OwnedBinary::new(ciphertext.len()).expect("allocation failed");
    bin.as_mut_slice().copy_from_slice(&ciphertext);

    Ok((atoms::ok(), Binary::from_owned(bin, env)).encode(env))
}

#[rustler::nif]
fn decrypt<'a>(env: Env<'a>, pem: Binary, data: Binary) -> Result<Binary<'a>, Atom> {
    let cursor = io::Cursor::new(pem.as_slice());
    let (key, _) = match SignedSecretKey::from_armor_single(cursor) {
        Ok(v) => v,
        _ => return Err(atoms::invalid_key()),
    };

    let message = match Message::from_bytes(data.as_slice()) {
        Ok(v) => v,
        _ => return Err(atoms::invalid_data()),
    };

    match &message {
        Message::Encrypted { .. } => {
            let decrypter = message.decrypt(|| "".to_string(), || "".to_string(), &[&key]);
            let (mut decrypter, _) = match decrypter {
                Ok(v) => v,
                _ => return Err(atoms::decryption_failed()),
            };

            let decrypted = match decrypter.next() {
                Some(next) => match next {
                    Ok(v) => v,
                    _ => return Err(atoms::decryption_failed()),
                },
                _ => return Err(atoms::decryption_failed()),
            };

            let raw = match decrypted {
                Message::Literal(data) => data,
                Message::Compressed(data) => {
                    let bytes = match data.decompress() {
                        Ok(v) => v,
                        _ => return Err(atoms::decryption_failed()),
                    };

                    let m = match Message::from_bytes(bytes) {
                        Ok(v) => v,
                        _ => return Err(atoms::decryption_failed()),
                    };
                    match m.get_literal() {
                        Some(v) => v.clone(),
                        _ => return Err(atoms::decryption_failed()),
                    }
                }
                _ => return Err(atoms::decryption_failed()),
            };

            let plaintext = raw.data();
            let mut bin = OwnedBinary::new(plaintext.len()).expect("allocation failed");
            bin.as_mut_slice().copy_from_slice(&plaintext);

            Ok(Binary::from_owned(bin, env))
        }
        _ => return Err(atoms::decryption_failed()),
    }
}

#[rustler::nif]
fn fingerprint<'a>(env: Env<'a>, pem: Binary) -> Result<Binary<'a>, Atom> {
    let cursor = io::Cursor::new(pem.as_slice());
    let (key, _) = match SignedSecretKey::from_armor_single(cursor) {
        Ok(v) => v,
        _ => return Err(atoms::invalid_key()),
    };

    let fingerprint = key.fingerprint();
    let mut bin = OwnedBinary::new(fingerprint.len()).expect("allocation failed");
    bin.as_mut_slice().copy_from_slice(&fingerprint);

    Ok(Binary::from_owned(bin, env))
}

rustler::init!(
    "Elixir.PGP",
    [
        generate,
        created_at,
        export_public_key,
        export_rsa_pubkey,
        encrypt,
        decrypt,
        fingerprint
    ]
);
