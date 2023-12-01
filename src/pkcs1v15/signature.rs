pub use ::signature::{
    hazmat::{PrehashSigner, PrehashVerifier},
    DigestSigner, DigestVerifier, Error, Keypair, RandomizedDigestSigner, RandomizedSigner, Result,
    SignatureEncoding, Signer, Verifier,
};
use crypto_bigint::BoxedUint;
use spki::{
    der::{asn1::BitString, Result as DerResult},
    SignatureBitStringEncoding,
};

use crate::algorithms::pad::uint_to_be_pad;
use alloc::{boxed::Box, string::ToString};
use core::fmt::{Debug, Display, Formatter, LowerHex, UpperHex};

/// `RSASSA-PKCS1-v1_5` signatures as described in [RFC8017 § 8.2].
///
/// [RFC8017 § 8.2]: https://datatracker.ietf.org/doc/html/rfc8017#section-8.2
#[derive(Clone, PartialEq, Eq)]
pub struct Signature {
    pub(super) inner: BoxedUint,
    pub(super) len: usize,
}

impl SignatureEncoding for Signature {
    type Repr = Box<[u8]>;
}

impl SignatureBitStringEncoding for Signature {
    fn to_bitstring(&self) -> DerResult<BitString> {
        BitString::new(0, self.to_vec())
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = signature::Error;

    fn try_from(bytes: &[u8]) -> signature::Result<Self> {
        let len = bytes.len();
        Ok(Self {
            // TODO: how to convert error?
            inner: BoxedUint::from_be_slice(bytes, len * 8).unwrap(),
            len,
        })
    }
}

impl From<Signature> for Box<[u8]> {
    fn from(signature: Signature) -> Box<[u8]> {
        uint_to_be_pad(signature.inner, signature.len)
            .expect("RSASSA-PKCS1-v1_5 length invariants should've been enforced")
            .into_boxed_slice()
    }
}

impl Debug for Signature {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        fmt.debug_tuple("Signature")
            .field(&self.to_string())
            .finish()
    }
}

impl LowerHex for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for byte in self.to_bytes().iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl UpperHex for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for byte in self.to_bytes().iter() {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:X}", self)
    }
}
