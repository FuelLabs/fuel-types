use crate::bytes;
use crate::types::hex_val;
use crate::TxId;

use core::array::TryFromSliceError;
use core::convert::TryFrom;
use core::{fmt, str};
use zerocopy::{AsBytes, FromBytes};

#[cfg(feature = "random")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[repr(packed)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsBytes, FromBytes)]
/// Fuel atomic type to represent identifier of the coin.
pub struct CoinId {
    tx_id: TxId,
    output_index: u8,
}

impl CoinId {
    /// Memory length of the type
    pub const LEN: usize = 33;

    /// Bytes constructor.
    pub const fn new(bytes: [u8; 33]) -> Self {
        let r = bytes;

        #[rustfmt::skip]
        let tx_id: [u8; TxId::LEN] = [
            r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7],
            r[8], r[9], r[10], r[11], r[12], r[13], r[14], r[15],
            r[16], r[17], r[18], r[19], r[20], r[21], r[22], r[23],
            r[24], r[25], r[26], r[27], r[28], r[29], r[30], r[31],
        ];

        let output_index = r[32];

        Self {
            tx_id: TxId::new(tx_id),
            output_index,
        }
    }

    /// Zeroes bytes constructor.
    pub const fn zeroed() -> Self {
        Self {
            tx_id: TxId::new([0; 32]),
            output_index: 0,
        }
    }

    /// Add a conversion from arbitrary slices into owned
    ///
    /// # Safety
    ///
    /// This function will not panic if the length of the slice is smaller than
    /// `Self::LEN`. Instead, it will cause undefined behavior and read random disowned
    /// bytes
    pub unsafe fn from_slice_unchecked(bytes: &[u8]) -> Self {
        Self::new(bytes::from_slice_unchecked(bytes))
    }

    /// Copy-free reference cast
    /// # Safety
    /// Assumes byte slice is the same length as this type.
    pub unsafe fn as_ref_unchecked(bytes: &[u8]) -> &Self {
        // The interpreter will frequently make references to keys and values using
        // logically checked slices.
        //
        // This function will save unnecessary copy to owned slices for the interpreter
        // access
        &*(bytes.as_ptr() as *const Self)
    }

    /// The memory size of the type by the method.
    pub const fn size(&self) -> usize {
        Self::LEN
    }

    /// Returns transaction id of the coin.
    pub const fn tx_id(&self) -> &TxId {
        &self.tx_id
    }

    /// Returns the output index in the transaction of the coin.
    pub const fn output_index(&self) -> u8 {
        self.output_index
    }

    /// Replaces transaction id.
    pub fn replace_tx_id(&mut self, tx_id: TxId) {
        self.tx_id = tx_id;
    }
}

#[cfg(feature = "random")]
impl rand::Fill for CoinId {
    fn try_fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) -> Result<(), rand::Error> {
        rng.fill_bytes(self.as_mut());

        Ok(())
    }
}

impl AsRef<[u8]> for CoinId {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsMut<[u8]> for CoinId {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}

impl From<[u8; 33]> for CoinId {
    fn from(bytes: [u8; 33]) -> Self {
        Self::new(bytes)
    }
}

impl From<CoinId> for [u8; 33] {
    fn from(salt: CoinId) -> [u8; 33] {
        salt.try_into().expect("Got and error during conversion")
    }
}

impl TryFrom<&[u8]> for CoinId {
    type Error = TryFromSliceError;

    fn try_from(bytes: &[u8]) -> Result<CoinId, TryFromSliceError> {
        Ok(CoinId::new(<[u8; 33]>::try_from(bytes).map(|b| b.into())?))
    }
}

impl fmt::LowerHex for CoinId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?
        }

        match f.width() {
            Some(w) if w > 0 => self
                .as_ref()
                .chunks(2 * Self::LEN / w)
                .try_for_each(|c| write!(f, "{:02x}", c.iter().fold(0u8, |acc, x| acc ^ x))),

            _ => self
                .as_ref()
                .iter()
                .try_for_each(|b| write!(f, "{:02x}", &b)),
        }
    }
}

impl fmt::UpperHex for CoinId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?
        }

        match f.width() {
            Some(w) if w > 0 => self
                .as_ref()
                .chunks(2 * Self::LEN / w)
                .try_for_each(|c| write!(f, "{:02X}", c.iter().fold(0u8, |acc, x| acc ^ x))),

            _ => self
                .as_ref()
                .iter()
                .try_for_each(|b| write!(f, "{:02X}", &b)),
        }
    }
}

impl fmt::Debug for CoinId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::LowerHex>::fmt(&self, f)
    }
}

impl fmt::Display for CoinId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::LowerHex>::fmt(&self, f)
    }
}

impl str::FromStr for CoinId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERR: &str = "Invalid encoded byte";

        let alternate = s.starts_with("0x");

        let mut b = s.bytes();
        let mut ret = CoinId::zeroed();

        if alternate {
            b.next();
            b.next();
        }

        for r in ret.as_mut() {
            let h = b.next().and_then(hex_val).ok_or(ERR)?;
            let l = b.next().and_then(hex_val).ok_or(ERR)?;

            *r = h << 4 | l;
        }

        Ok(ret)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for CoinId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:x}", &self))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CoinId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        s.parse().map_err(D::Error::custom)
    }
}

#[cfg(feature = "random")]
impl Distribution<CoinId> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CoinId {
        CoinId {
            tx_id: rng.gen(),
            output_index: rng.gen(),
        }
    }
}
