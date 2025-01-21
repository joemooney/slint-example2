// use mvc::PowerModel;

use std::rc::Rc;

use slint::Model;
// use slint::ModelExt;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::ui;
use crate::mvc;
use super::PowerStruct;

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

#[derive(Clone)]
pub struct PowerPresetModel {
    mission_repo: Rc<dyn mvc::traits::MissionRepository + 'static>,
    repo: Rc<dyn mvc::traits::PowerPresetRepository>,
    notify: Rc<ModelNotify>,
    pub preset_names : Rc<slint::VecModel<slint::SharedString>>,
}

impl PowerPresetModel {
    pub fn new(repo: Rc<dyn mvc::traits::PowerPresetRepository + 'static>, mission_repo: Rc<dyn mvc::traits::MissionRepository + 'static>) -> Self {
        // let name1 = SharedString::from("aaa");
        let preset_names = Rc::new(slint::VecModel::from(vec![]));
        let preset_model = Self { 
            repo,
            mission_repo,
            notify: Rc::new(Default::default()) ,
            preset_names,
        };
        preset_model.update_preset_names();
        preset_model
    }

    // connects repo to a Slint `Model`` of Vec<MissionSlintStruct>
    pub fn ui_mapping(&self) -> ModelRc<crate::ui::PowerPresetSlintStruct> {
        let wrapped_model: ModelRc<mvc::PowerPresetStruct> = ModelRc::new(self.clone());
        let map_function = mvc::PowerPresetStruct::map_power_preset_to_slint;
        let connector: ModelRc<crate::ui::PowerPresetSlintStruct> = Rc::new(
            slint::MapModel::new(wrapped_model, map_function)
        ).into();
        connector
    }

    // fn get_preset_names1(names: Vec<String>) -> Rc<slint::VecModel<slint::SharedString>> {
    //     let v: slint::VecModel<slint::SharedString> = names.into_iter().map(|s| slint::SharedString::from(s)).collect();
    //     Rc::new(v)
    // }

    fn get_preset_names(names: Vec<String>) -> Vec<slint::SharedString> {
        let mut v: Vec<slint::SharedString> = names.into_iter().map(|s| slint::SharedString::from(s)).collect();
        v.push(slint::SharedString::from("Custom"));
        v
    }

    // fn toggle_done(&self, index: usize) {
    //     if !self.repo.toggle_done(index) {
    //         return;
    //     }

    //     self.notify.row_changed(index)
    // }

    fn get_preset(&self, index: usize) -> Option<PowerPresetStruct> {
        println!("[PowerPresetModel] getting preset {index}");
        self.repo.get_power_preset(index)
    }

    pub fn remove_preset(&self, index: usize) {
        println!("[PowerPresetModel] remove preset {index}");
        if !self.repo.remove_power_preset(index) {
            return;
        }
        // self.preset_names.set_vec(self.repo.preset_names().iter().map(|s| s.into()).collect());
        self.update_preset_names();
        self.notify.row_removed(index, 1)
    }

    fn update_preset_names(&self) {
        self.preset_names.set_vec(PowerPresetModel::get_preset_names(self.repo.preset_names()));
    }

    pub fn update_preset(&self, index: usize, preset: ui::PowerPresetSlintStruct) {
        println!("[PowerPresetModel] update preset {index}");
        if !self.repo.update_power_preset(index, preset.into()) {
            return;
        }
        self.update_preset_names();
        self.notify.row_changed(index);
    }

    pub fn create_preset(&self, preset: ui::PowerPresetSlintStruct) {
        println!("[PowerPresetModel] push preset");
        if !self.repo.push_power_preset(preset.into()) {
            return;
        }
        self.update_preset_names();
        self.notify.row_added(self.row_count() - 1, 1);
    }

    pub fn change_preset_int_value(&self, mut preset: ui::PowerPresetSlintStruct, field_name: impl AsRef<str>, value: i32) -> ui::PowerPresetSlintStruct {
        match field_name.as_ref() {
            "power9" => preset.power9 = value,
            "power10" => preset.power10 = value,
            _ => {}
        }
        preset
    }
    pub fn change_preset_float_value(&self, mut preset: ui::PowerPresetSlintStruct, field_name: impl AsRef<str>, value: f32) -> ui::PowerPresetSlintStruct {
        match field_name.as_ref() {
            "power1" => preset.power1 = value,
            "power2" => preset.power2 = value,
            "power3" => preset.power3 = value,
            "power4" => preset.power4 = value,
            "power5" => preset.power5 = value,
            "power6" => preset.power6 = value,
            "power7" => preset.power7 = value,
            "power8" => preset.power8 = value,
            _ => {}
        }
        preset
    }
    pub fn check_preset_field(&self, mut preset: ui::PowerPresetSlintStruct, field_name: impl AsRef<str>, value: impl AsRef<str>) -> ui::PowerPresetSlintStruct {
        match field_name.as_ref() {
            "preset_name" => preset.preset_name = value.as_ref().to_owned().into(),
            "preset_desc" => preset.preset_desc = value.as_ref().to_owned().into(),
            _ => {}
        }
        preset
    }
}

impl Model for PowerPresetModel {
    type Data = PowerPresetStruct;

    fn row_count(&self) -> usize {
        self.repo.power_preset_count()
    }

    fn row_data(&self, index: usize) -> Option<Self::Data> {
        self.repo.get_power_preset(index)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mvc::{self, PowerPresetStruct, PowerStruct};

    fn power_preset1() -> PowerPresetStruct {
        let power1 = PowerStruct{power1: 1.1, power2: 1.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1};
        let power_preset1 = PowerPresetStruct{power_preset_name: "power1".into(), power_preset_desc: "desc".into(), values: power1 };
        power_preset1
    }
    fn power_preset2() -> PowerPresetStruct {
        let power2 = PowerStruct{power1: 2.2, power2: 2.2, power3: 3.2, power4: 2.2, power5: 2.2, power6: 2.2, power7: 2.2, power8: 2.2, power9: 2, power10: 2};
        let power_preset2 = PowerPresetStruct{power_preset_name: "power2".into(), power_preset_desc: "desc".into(), values: power2 };
        power_preset2
    }
    fn power_preset3() -> PowerPresetStruct {
        let power3 = PowerStruct{power1: 3.3, power2: 3.3, power3: 3.3, power4: 3.3, power5: 3.3, power6: 3.3, power7: 3.3, power8: 3.3, power9: 3, power10: 3};
        let power_preset3 = PowerPresetStruct{power_preset_name: "power3".into(), power_preset_desc: "desc".into(), values: power3 };
        power_preset3
    }

    fn test_model() -> PowerPresetModel {
        let repo = mvc::MockPowerPresetRepository::new(vec![
            power_preset1(),
            power_preset2(),
        ]);
        let mission_repo = mvc::MockMissionRepository::new(vec![
            mvc::mission_tests::mission1(),
            mvc::mission_tests::mission2(),
        ]);
        PowerPresetModel::new(Rc::new(repo), Rc::new(mission_repo))
    }

    #[test]
    fn test_presets() {
        let preset_model = test_model();

        assert_eq!(preset_model.row_count(), 2);
        assert_eq!(
            preset_model.row_data(0),
            Some( power_preset1())
        );
        assert_eq!(
            preset_model.row_data(1),
            Some( power_preset2())
        );
    }

    #[test]
    fn test_remove_preset() {
        let preset_model = test_model();

        assert_eq!(preset_model.row_count(), 2);
        preset_model.remove_preset(0);
        assert_eq!(preset_model.row_count(), 1);

        assert_eq!(
            preset_model.row_data(0),
            Some( power_preset2())
        );
    }

    #[test]
    fn test_add_preset() {
        let preset_model = test_model();

        assert_eq!(preset_model.row_count(), 2);
        preset_model.create_preset(power_preset3().into());
        assert_eq!(preset_model.row_count(), 3);

        assert_eq!(
            preset_model.row_data(2),
            Some( power_preset3())
        );
    }
}
