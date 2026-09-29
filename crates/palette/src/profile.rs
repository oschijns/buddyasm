use crate::profile::constants::*;
use buddyasm_common::{
    color_space::{Color, ColorSpace},
    system::System,
};

/// Raw data for constructing palettes
mod constants;

/// Create a color palette for the specified system
#[rustfmt::skip]
pub fn make_color_palette(system: System) -> ColorSpace {
    let list: &[(Color, usize)] = match system {
        System::Famicom        => &PALETTE_NES,
        System::SuperFamicom   => todo!(),
        System::GameBoy        => &PALETTE_GB,
        System::GameBoyColor   => todo!(),
        System::VirtualBoy     => todo!(),
        System::GameBoyAdvance => todo!(),
        System::PcEngine       => todo!(),
        System::WonderSwan     => todo!(),
        System::MasterSystem   => &PALETTE_SMS,
        System::MegaDrive      => todo!(),
        System::NeoGeoPocket   => todo!(),
        System::NeoGeo         => todo!(),
    };
    ColorSpace::load(list)
}
