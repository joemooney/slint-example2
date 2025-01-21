use crate::mvc;

pub trait FrequencyPresetRepository {
    // fn get_preset_by_idx(&self, index: usize) -> Option<mvc::FrequencyPresetModel>;
    // fn get_preset_by_name(&self, preset_name: &str) -> Option<mvc::FrequencyPresetModel>;
    // fn set_freq1(&self, preset_name: &str, freq1: f32) -> bool;
    // fn remove_preset_by_name(&self, preset_naem: &str) -> bool;
    // fn remove_preset_by_idx(&self, index: usize) -> bool;
    // fn push_preset(&self, preset: mvc::FrequencyPresetModel) -> bool;
    // fn set_freq1(&self, preset_name: &str, freq1: f32) -> bool;
    fn preset_names(&self) -> Vec<String>;
    fn frequency_preset_count(&self) -> usize;
    fn get_frequency_preset(&self, index: usize) -> Option<mvc::FrequencyPresetStruct>;
    fn find_power_preset(&self, preset_name: &str) -> Option<mvc::FrequencyPresetStruct>;
    fn remove_frequency_preset(&self, index: usize) -> bool;
    fn push_frequency_preset(&self, frequency_preset: mvc::FrequencyPresetStruct) -> bool;
    fn update_frequency_preset(&self, index: usize, frequency_preset: mvc::FrequencyPresetStruct) -> bool;
}
