use deno_crypto::RawKeyData;
use deno_crypto::SharedError;

#[test]
fn raw_key_data_preserves_secret_material_and_key_kind() {
  let key = RawKeyData::Secret(vec![1, 2, 3, 4].into_boxed_slice());

  assert_eq!(key.bytes(), &[1, 2, 3, 4]);
  assert_eq!(key.as_secret_key().unwrap(), &[1, 2, 3, 4]);
  assert!(matches!(
    key.as_rsa_private_key(),
    Err(SharedError::ExpectedPrivateKey)
  ));
  assert!(matches!(
    key.as_rsa_public_key(),
    Err(SharedError::ExpectedPublicKey)
  ));
}

#[test]
fn seeded_private_keys_expose_seed_and_expanded_key_separately() {
  let key = RawKeyData::SeededPrivate {
    seed: Some(vec![9, 8].into_boxed_slice()),
    private_key: vec![7, 6, 5].into_boxed_slice(),
  };

  assert_eq!(key.seed(), Some(&[9, 8][..]));
  assert_eq!(key.expanded_private_key(), &[7, 6, 5]);
}

#[test]
fn public_key_material_is_returned_without_copying() {
  let key = RawKeyData::Public(vec![0x01, 0x02].into_boxed_slice());
  let public_key = key.as_rsa_public_key().unwrap();
  assert!(matches!(public_key, std::borrow::Cow::Borrowed(_)));
  assert_eq!(public_key.as_ref(), &[0x01, 0x02]);
}
