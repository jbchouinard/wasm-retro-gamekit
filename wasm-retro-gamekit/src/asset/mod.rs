use bincode::error::DecodeError;
use bincode::{config, decode_from_slice, Decode, Encode};

use self::image::{CompressedCm4Image, CompressedRgbaImage};

pub mod image;

#[derive(Encode, Decode)]
pub enum Asset {
    RgbaImage(CompressedRgbaImage),
    Cm4Image(CompressedCm4Image),
}

impl Asset {
    fn marker(&self) -> String {
        match self {
            Asset::RgbaImage(_) => "WRG.IMG.RGBA.1\r\n",
            Asset::Cm4Image(_) => "WRG.IMG.CM4.1\r\n",
        }
        .to_string()
    }

    pub fn into_blob(self) -> AssetBlob {
        let marker = self.marker();
        AssetBlob {
            asset: self,
            marker,
        }
    }
}

#[derive(Encode, Decode)]
pub struct AssetBlob {
    marker: String,
    asset: Asset,
}

impl AssetBlob {
    pub fn into_asset(self) -> Asset {
        self.asset
    }
}

pub fn load_asset(asset: &[u8]) -> Result<(AssetBlob, usize), DecodeError> {
    decode_from_slice(asset, config::standard())
}
