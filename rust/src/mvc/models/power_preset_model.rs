// use mvc::PowerModel;

use super::PowerStruct;
use crate::ui;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct PowerPresetStruct {
    pub power_preset_name: String,
    // pub power_preset_id: usize,
    pub power_preset_desc: String,
    pub values: PowerStruct,
}

impl PowerPresetStruct {
    // maps a PowerPresetModel (data) to a PowerPresetSlintStruct (ui)
    pub fn map_power_preset_to_slint(preset: PowerPresetStruct) -> ui::PowerPresetSlintStruct {
        preset.into()
    }

    // maps a PowerPresetSlintStruct (data) to a PowerPresetModel (ui)
    pub fn map_power_preset_from_slint(preset: ui::PowerPresetSlintStruct) -> PowerPresetStruct {
        preset.into()
    }
}
impl From<PowerPresetStruct> for ui::PowerPresetSlintStruct {
    fn from(preset: PowerPresetStruct) -> Self {
        ui::PowerPresetSlintStruct {
            preset_name: preset.power_preset_name.into(),
            preset_desc: preset.power_preset_desc.into(),
            preset_details: std::format!("power1:{} power2:{}",preset.values.power1,preset.values.power2).into(),
            checked: false,
            power1: preset.values.power1,
            power2: preset.values.power2,
            power3: preset.values.power3,
            power4: preset.values.power4,
            power5: preset.values.power5,
            power6: preset.values.power6,
            power7: preset.values.power7,
            power8: preset.values.power8,
            power9: preset.values.power9 as i32,
            power10: preset.values.power10 as i32,
        }
    }
}

impl From<ui::PowerPresetSlintStruct> for PowerPresetStruct {
    fn from(preset: ui::PowerPresetSlintStruct) -> Self {
        let values = PowerStruct{
            power1: preset.power1,
            power2: preset.power2,
            power3: preset.power3,
            power4: preset.power4,
            power5: preset.power5,
            power6: preset.power6,
            power7: preset.power7,
            power8: preset.power8,
            power9: preset.power9 as u32,
            power10: preset.power10 as u32,
        };
        PowerPresetStruct {
            power_preset_name: preset.preset_name.into(),
            power_preset_desc: preset.preset_desc.into(),
            values,
        }
    }
}