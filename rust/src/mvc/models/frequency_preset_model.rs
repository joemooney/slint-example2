use crate::util;
use crate::ui;

use super::FrequencyStruct;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct FrequencyPresetStruct {
    // pub preset_id: usize,
    pub frequency_preset_name: String,
    pub frequency_preset_desc: String,
    pub values: FrequencyStruct,
}

impl FrequencyPresetStruct {
    // maps a FrequencyPresetModel (data) to a FrequencyPresetSlintStruct (ui)
    pub fn map_frequency_preset_from_slint(preset: ui::FrequencyPresetSlintStruct) -> FrequencyPresetStruct {
        preset.into()
    }
    // maps a FrequencyPresetModel (data) to a FrequencyPresetSlintStruct (ui)
    pub fn map_frequency_preset_to_slint(preset: FrequencyPresetStruct) -> ui::FrequencyPresetSlintStruct {
        preset.into()
    }
}

impl From<FrequencyPresetStruct> for ui::FrequencyPresetSlintStruct {
    fn from(preset: FrequencyPresetStruct) -> Self {
        ui::FrequencyPresetSlintStruct {
            preset_name: preset.frequency_preset_name.into(),
            preset_desc: preset.frequency_preset_desc.into(),
            preset_details: std::format!("frequency1:{} frequency2:{:#?}",preset.values.freq1,preset.values.freq2).into(),
            freq1: preset.values.freq1,
            freq2: preset.values.freq2,
            freq3: util::float_list_to_string(&preset.values.freq3).into(),
            checked: false,
        }
    }
}

impl From<ui::FrequencyPresetSlintStruct> for FrequencyPresetStruct {
    fn from(preset: ui::FrequencyPresetSlintStruct) -> Self {
        let values = FrequencyStruct {
            freq1: preset.freq1,
            freq2: preset.freq2,
            freq3: util::string_to_float_list(preset.freq3.as_str()),
        };
        FrequencyPresetStruct {
            frequency_preset_name: preset.preset_name.into(),
            frequency_preset_desc: preset.preset_desc.into(),
            values,
        }
    }
}
