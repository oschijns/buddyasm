use crate::{input_stack::InputConfig, profile::*};

/// Get a builder configuration from the selected hardware profile
pub trait ToConfig {
    fn to_config(&self) -> InputConfig;
}

impl ToConfig for Profile {
    #[rustfmt::skip]
    fn to_config(&self) -> InputConfig {
        match self {
            Self::Famicom     (profile) => profile.to_config(),
            Self::SuperFamicom(profile) => profile.to_config(),
            Self::GameBoy     (profile) => profile.to_config(),
            Self::GameBoyColor(profile) => profile.to_config(),
            Self::VirtualBoy  (profile) => profile.to_config(),
            Self::PcEngine    (profile) => profile.to_config(),
            Self::WonderSwan  (profile) => profile.to_config(),
            Self::MasterSystem(profile) => profile.to_config(),
            Self::MegaDrive   (profile) => profile.to_config(),
            Self::NeoGeoPocket(profile) => profile.to_config(),
            Self::NeoGeo      (profile) => profile.to_config(),
        }
    }
}

macro_rules! impl_to_config {
    ($profile:ty) => {
        impl ToConfig for $profile {
            #[inline]
            fn to_config(&self) -> InputConfig {
                InputConfig {
                    tile_count: 256,
                    tile_size: self.mode.tile_size(),
                    flip: self.mode.tile_flip(),
                }
            }
        }
    };
}

impl_to_config!(ProfileFamicom);
impl_to_config!(ProfileSuperFamicom);
impl_to_config!(ProfileGameBoy);
impl_to_config!(ProfileGameBoyColor);
impl_to_config!(ProfileVirtualBoy);
impl_to_config!(ProfilePcEngine);
impl_to_config!(ProfileWonderSwan);
impl_to_config!(ProfileMasterSystem);
impl_to_config!(ProfileMegaDrive);
impl_to_config!(ProfileNeoGeoPocket);
impl_to_config!(ProfileNeoGeo);
