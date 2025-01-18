// use std::{cell::RefCell, collections::HashMap, rc::Rc};
use std::{cell::RefCell, rc::Rc};

use super::traits;
use crate::mvc;

#[derive(Clone)]
pub struct MockPowerPresetRepository {
    // presets: Rc<RefCell<HashMap<String, mvc::PowerPresetStruct>>>,
    power_presets: Rc<RefCell<Vec<mvc::PowerPresetStruct>>>,
}

impl MockPowerPresetRepository {
    // pub fn new(presets: HashMap<String, mvc::PowerPresetStruct>) -> Self {
    pub fn new(power_presets: Vec<mvc::PowerPresetStruct>) -> Self {
        Self { power_presets: Rc::new(RefCell::new(power_presets)) }
    }
}

impl traits::PowerPresetRepository for MockPowerPresetRepository {
    fn power_preset_count(&self) -> usize {
        self.power_presets.borrow().len()
    }

    fn get_power_preset(&self, index: usize) -> Option<mvc::PowerPresetStruct> {
        self.power_presets.borrow().get(index).cloned()
    }

    // fn toggle_flagged(&self, index: usize) -> bool {
    //     if let Some(power_preset) = self.missions.borrow_mut().get_mut(index) {
    //         power_preset.flagged = !preset.flagged;
    //         return true;
    //     }

    //     false
    // }

    fn remove_power_preset(&self, index: usize) -> bool {
        if index < self.power_presets.borrow().len() {
            self.power_presets.borrow_mut().remove(index);
            return true;
        }

        false
    }

    fn push_power_preset(&self, preset: mvc::PowerPresetStruct) -> bool {
        self.power_presets.borrow_mut().push(preset);
        true
    }

    fn update_power_preset(&self, index: usize, preset: mvc::PowerPresetStruct) -> bool {
        // self.power_presets.borrow_mut().push(preset);
        if index < self.power_presets.borrow().len() {
            self.power_presets.borrow_mut()[index] = preset;
            return true
        }
        false
    }

    // fn set_power1(&self, preset_name: &str, power1: f32) -> bool {
    //     if let Some(preset) = self.presets.borrow_mut().get_mut(preset_name) {
    //         preset.values.power1 = power1;
    //         return true;
    //     }
    //     false
    // }

    // fn get_preset_by_name(&self, preset_name: &str) -> Option<mvc::PowerPresetStruct> {
    //     self.presets.borrow().get(preset_name).cloned()
    // }

    // fn get_preset_by_idx(&self, index: usize) -> Option<mvc::PowerPresetStruct> {
    //     self.presets.borrow().values().filter(|x| x.preset_id == index).next().map(|x| x.to_owned())
    // }

    // fn remove_preset_by_name(&self, preset_name: &str) -> bool {
    //     if let Some(_) = self.presets.borrow().get(preset_name) {
    //         self.presets.borrow_mut().remove(preset_name);
    //         return true;
    //     }
    //     false
    // }
    // fn get_preset_by_name(&self, preset_name: &str) -> Option<mvc::PowerPresetStruct> {
    //     self.presets.borrow().get(preset_name).cloned()
    // }

    // fn get_preset_by_idx(&self, index: usize) -> Option<mvc::PowerPresetStruct> {
    //     self.presets.borrow().values().filter(|x| x.preset_id == index).next().map(|x| x.to_owned())
    // }


    // fn remove_preset_by_idx(&self, index: usize) -> bool {
    //     let preset = self.presets.borrow().values().filter(|x| x.preset_id == index).next().map(|x| x.to_owned());
    //     if let Some(preset) = preset {
    //         return self.remove_preset_by_name(&preset.preset_name);
    //     }
    //     false
    // }

    // fn push_preset(&self, preset: mvc::PowerPresetStruct) -> bool {
    //     self.presets.borrow_mut().insert(preset.preset_name.to_owned(), preset);
    //     true
    // }
}
