use super::{Engine, SpecTransform};
use crate::pb::*;
use bytes::Bytes;
use lazy_static::lazy_static;
use photon_rs::{PhotonImage, native::open_image_from_bytes, transform};

lazy_static! {
    static ref WATERMARK: PhotonImage = {
        let data = include_bytes!("../../rust-logo.png");
        let wartermark = open_image_from_bytes(data).unwrap();
        transform::resize(&wartermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

pub struct Photon(PhotonImage);

impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&value)?))
    }
}
