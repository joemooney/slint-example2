use super::{FrequencyPresetStruct, PowerPresetStruct};
use crate::ui;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct MissionStruct {
    pub mission_name: String,
    pub mission_desc: String,
    pub mission_id: i32,
    // pub frequency_preset: String,
    // pub power_preset: String,
    pub frequency_model: FrequencyPresetStruct,
    pub power_model: PowerPresetStruct,
    pub flagged: bool,

    // creation date in milliseconds i64
}

impl From<MissionStruct> for ui::MissionSlintStruct {
    fn from(mission: MissionStruct) -> Self {
        let power_model = PowerPresetStruct::map_power_preset_to_slint(mission.power_model);
        let frequency_model = FrequencyPresetStruct::map_frequency_preset_to_slint(mission.frequency_model);
        ui::MissionSlintStruct {
            mission_name: mission.mission_name.into(),
            mission_desc: mission.mission_desc.into(),
            mission_id: mission.mission_id,
            mission_details: std::format!("{} power:{}",frequency_model.preset_details,power_model.preset_name).into(),
            power_model,
            frequency_model,
            checked: false,
        }
    }
}

impl From<ui::MissionSlintStruct> for MissionStruct {
    fn from(mission: ui::MissionSlintStruct) -> Self {
        let power_model = mission.power_model.into();
        let frequency_model = mission.frequency_model.into();
        MissionStruct {
            mission_name: mission.mission_name.into(),
            mission_desc: mission.mission_desc.into(),
            mission_id: mission.mission_id,
            power_model,
            flagged: false,
            frequency_model,
        }
    }
}

impl MissionStruct {
    // maps a MissionModel (data) to a MissionSlintStruct (ui)
    pub fn map_mission_to_slint(mission: MissionStruct) -> ui::MissionSlintStruct {
        mission.into()
    }
}