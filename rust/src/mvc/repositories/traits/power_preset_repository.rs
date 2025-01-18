use crate::mvc;

pub trait PowerPresetRepository {
    // fn get_preset_by_name(&self, preset_name: &str) -> Option<mvc::PowerPresetModel>;
    // fn get_preset_by_idx(&self, index: usize) -> Option<mvc::PowerPresetModel>;
    // fn set_power1(&self, preset_name: &str, power1: f32) -> bool;
    // fn remove_preset_by_name(&self, preset_name: &str) -> bool;
    // fn remove_preset_by_idx(&self, index: usize) -> bool;
    fn power_preset_count(&self) -> usize;
    fn get_power_preset(&self, index: usize) -> Option<mvc::PowerPresetStruct>;
    fn remove_power_preset(&self, index: usize) -> bool;
    fn push_power_preset(&self, power_preset: mvc::PowerPresetStruct) -> bool;
    fn update_power_preset(&self, index: usize, power_preset: mvc::PowerPresetStruct) -> bool;
}
