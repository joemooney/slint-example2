
use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

// use crate::ui;
use crate::ui;
use crate::mvc;
use crate::mvc::FrequencyPresetStruct;

// use crate::mvc::FrequencyPresetStruct;
// use crate::mvc::FrequencyPresetModel;
// use crate::util;
// use crate::Callback;

// #[derive(Clone)]
// pub struct FrequencyPresetListController {
//     preset_model: FrequencyPresetModel,
//     // show_create_frequency_preset_callback: Rc<Callback<(), ()>>,
//     // show_edit_frequency_preset_callback: Rc<Callback<(), ()>>,
// }

// impl FrequencyPresetListController {
//     pub fn new(repo: impl mvc::traits::FrequencyPresetRepository + 'static) -> Self {
//         Self {
//             preset_model: FrequencyPresetModel::new(repo),
//             // show_create_frequency_preset_callback: Rc::new(Callback::default()),
//             // show_edit_frequency_preset_callback: Rc::new(Callback::default()),
//         }
//     }

//     pub fn preset_model(&self) -> ModelRc<mvc::FrequencyPresetStruct> {
//         ModelRc::new(self.preset_model.clone())
//     }

//     // pub fn toggle_done(&self, index: usize) {
//     //     self.preset_model.toggle_done(index)
//     // }

//     pub fn get_preset(&self, index: usize) -> Option<mvc::FrequencyPresetStruct> {
//         self.preset_model.get_preset(index)
//     }

//     pub fn remove_preset(&self, index: usize) {
//         self.preset_model.remove_preset(index)
//     }

//     pub fn update_preset(&self, index: usize, preset_name: &str, preset_desc: &str, freq1: f32, freq2: f32, freq3: &str) {
//         println!("controllers/frequency_preset_list_controller:create_preset");
//         // let preset_id = self.preset_model.
//         let freq3_csv = crate::util::string_to_float_list(freq3);
//         self.preset_model.update_preset(index, mvc::FrequencyPresetStruct {
//             frequency_preset_name: preset_name.into(),
//             frequency_preset_desc: preset_desc.into(),
//             values: FrequencyStruct{ freq1, freq2, freq3: freq3_csv },
//             ..Default::default()
//         })
//     }

//     // pub fn create_preset(&self, preset_name: &str, preset_desc: &str, freq1: f32, freq2: f32, freq3: Vec<f32>) {
//     pub fn create_preset(&self, preset: mvc::FrequencyPresetStruct) {
//         // self.preset_model.push_preset(mvc::FrequencyPresetModel {
//         //     frequency_preset_name: preset_name.into(),
//         //     frequency_preset_desc: preset_desc.into(),
//         //     values: FrequencyModel{ freq1, freq2, freq3 },
//         //     ..Default::default()
//         // })
//         self.preset_model.push_preset(preset)
//     }

//     // pub fn show_edit_frequency_preset(&self) {
//     //     println!("frequency_preset_list_controller.rs show_edit_frequency_preset_callback.invoke");
//     //     self.show_edit_frequency_preset_callback.invoke(&());
//     // }

//     // pub fn on_show_edit_frequency_preset(&self, mut callback: impl FnMut() + 'static) {
//     //     println!("frequency_preset_list_controller.rs on_show_edit_frequency_preset");
//     //     self.show_edit_frequency_preset_callback.on(move |()| {
//     //         callback();
//     //     });
//     // }

//     // pub fn show_create_frequency_preset(&self) {
//     //     self.show_create_frequency_preset_callback.invoke(&());
//     // }

//     // pub fn on_show_create_frequency_preset(&self, mut callback: impl FnMut() + 'static) {
//     //     self.show_create_frequency_preset_callback.on(move |()| {
//     //         callback();
//     //     });
//     // }
// }

#[derive(Clone)]
pub struct FrequencyPresetModel {
    mission_repo: Rc<dyn mvc::traits::MissionRepository + 'static>,
    repo: Rc<dyn mvc::traits::FrequencyPresetRepository>,
    notify: Rc<ModelNotify>,
    pub preset_names : Rc<slint::VecModel<slint::SharedString>>,
}

impl FrequencyPresetModel {
    pub fn new(repo: Rc<dyn mvc::traits::FrequencyPresetRepository + 'static>, mission_repo: Rc<dyn mvc::traits::MissionRepository + 'static>) -> Self {
        // Self { repo: Rc::new(repo), notify: Rc::new(Default::default()) }
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
     pub fn ui_mapping(&self) -> ModelRc<crate::ui::FrequencyPresetSlintStruct> {
        let wrapped_model: ModelRc<mvc::FrequencyPresetStruct> = ModelRc::new(self.clone());
        let map_function = mvc::FrequencyPresetStruct::map_frequency_preset_to_slint;
        let connector: ModelRc<crate::ui::FrequencyPresetSlintStruct> = Rc::new(
            slint::MapModel::new(wrapped_model, map_function)
        ).into();
        connector
    }   

    pub fn get_preset_names(names: Vec<String>) -> Vec<slint::SharedString> {
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
    pub fn get_preset(&self, index: usize) -> Option<FrequencyPresetStruct> {
        self.repo.get_frequency_preset(index)
    }

    fn update_preset_names(&self) {
        self.preset_names.set_vec(FrequencyPresetModel::get_preset_names(self.repo.preset_names()));
    }

    pub fn remove_preset(&self, index: usize) {
        if !self.repo.remove_frequency_preset(index) {
            return;
        }

        self.update_preset_names();
        self.notify.row_removed(index, 1)
    }
    pub fn update_preset(&self, index: usize, preset: ui::FrequencyPresetSlintStruct) {
        if !self.repo.update_frequency_preset(index, preset.into()) {
            return;
        }
        self.notify.row_changed(index);
    }
    pub fn create_preset(&self, preset: ui::FrequencyPresetSlintStruct) {
        if !self.repo.push_frequency_preset(preset.into()) {
            return;
        }

        self.notify.row_added(self.row_count() - 1, 1);
    }
    pub fn change_preset_float_values(&self, mut preset: ui::FrequencyPresetSlintStruct, field_name: impl AsRef<str>, value: impl AsRef<str>) -> ui::FrequencyPresetSlintStruct {
        match field_name.as_ref() {
            "freq3" => preset.freq3 = value.as_ref().to_owned().into(),
            _ => {}
        }
        preset
    }
    pub fn change_preset_float_value(&self, mut preset: ui::FrequencyPresetSlintStruct, field_name: impl AsRef<str>, value: f32) -> ui::FrequencyPresetSlintStruct {
        match field_name.as_ref() {
            "freq1" => preset.freq1 = value,
            "freq2" => preset.freq2 = value,
            _ => {}
        }
        preset
    }
    pub fn check_preset_field(&self, mut preset: ui::FrequencyPresetSlintStruct, field_name: impl AsRef<str>, value: impl AsRef<str>) -> ui::FrequencyPresetSlintStruct {
        match field_name.as_ref() {
            "preset_name" => preset.preset_name = value.as_ref().to_owned().into(),
            "preset_desc" => preset.preset_desc = value.as_ref().to_owned().into(),
            _ => {}
        }
        preset
    }

}

impl Model for FrequencyPresetModel {
    type Data = mvc::FrequencyPresetStruct;

    fn row_count(&self) -> usize {
        self.repo.frequency_preset_count()
    }

    fn row_data(&self, index: usize) -> Option<Self::Data> {
        self.repo.get_frequency_preset(index)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mvc::{self, FrequencyPresetStruct, FrequencyStruct};
    // use std::cell::Cell;
    // use std::collections::HashMap;
    fn frequency_preset1() -> FrequencyPresetStruct {
        let frequency1 = FrequencyStruct{freq1: 1.1, freq2: 1.1, freq3: vec![1.1]};
        let frequency_preset1 = FrequencyPresetStruct{frequency_preset_name: "frequency1".into(), frequency_preset_desc: "desc".into(), values: frequency1 };
        frequency_preset1
    }
    fn frequency_preset2() -> FrequencyPresetStruct {
        let frequency2 = FrequencyStruct{freq1: 2.2, freq2: 2.2, freq3: vec![2.2]};
        let frequency_preset2 = FrequencyPresetStruct{frequency_preset_name: "frequency2".into(), frequency_preset_desc: "desc".into(), values: frequency2 };
        frequency_preset2
    }
    fn frequency_preset3() -> FrequencyPresetStruct {
        let frequency3 = FrequencyStruct{freq1: 3.3, freq2: 3.3, freq3: vec![3.3]};
        let frequency_preset3 = FrequencyPresetStruct{frequency_preset_name: "frequency3".into(), frequency_preset_desc: "desc".into(), values: frequency3 };
        frequency_preset3
    }
    fn test_model() -> FrequencyPresetModel {
        // FrequencyPresetListController::new(mvc::MockFrequencyPresetRepository::new(HashMap::from([
        //     ("frequency1".to_owned(), mvc::FrequencyPresetModel { 
        //         preset_name: "frequency1".into(),
        //         preset_desc: "frequency1".into(),
        //         preset_id: 1,
        //         values: FrequencyModel{ freq1: 1.0, freq2: 1.1, freq3: vec![2.0] }
        //         }),
        //     ("frequency2".to_owned(), mvc::FrequencyPresetModel {
        //         preset_name: "frequency2".into(),
        //         preset_desc: "frequency2".into(),
        //         preset_id: 2,
        //         values: FrequencyModel{ freq1: 1.0, freq2: 1.1, freq3: vec![2.0] }
        //         })
        // ])))
        let repo = mvc::MockFrequencyPresetRepository::new(vec![
            frequency_preset1(),
            frequency_preset2(),
        ]);
        let mission_repo = mvc::MockMissionRepository::new(vec![
            mvc::mission_tests::mission1(),
            mvc::mission_tests::mission2(),
        ]);
        FrequencyPresetModel::new(Rc::new(repo), Rc::new(mission_repo))        
    }

    #[test]
    fn test_presets() {
        let preset_model = test_model(); 

        assert_eq!(preset_model.row_count(), 2);
        assert_eq!(
            preset_model.row_data(0),
            Some(frequency_preset1())
        );
        assert_eq!(
            preset_model.row_data(1),
            Some(frequency_preset2())
        );
    }

    // #[test]
    // fn test_toggle_preset_checked() {
    //     let controller = test_controller();
    //     let preset_model = controller.preset_model();

    //     assert!(preset_model.row_data(0).unwrap().done);
    //     controller.toggle_done(0);
    //     assert!(!preset_model.row_data(0).unwrap().done);
    // }

    #[test]
    fn test_remove_preset() {
        let preset_model = test_model();

        assert_eq!(preset_model.row_count(), 2);
        preset_model.remove_preset(0);
        assert_eq!(preset_model.row_count(), 1);

        assert_eq!(
            preset_model.row_data(0),
            Some(frequency_preset2())
        );
    }

    // #[test]
    // fn test_show_create_frequency_preset() {
    //     let controller = test_controller();

    //     let callback_invoked = Rc::new(Cell::new(false));

    //     controller.on_show_create_frequency_preset({
    //         let callback_invoked = callback_invoked.clone();

    //         move || {
    //             callback_invoked.set(true);
    //         }
    //     });

    //     controller.show_create_frequency_preset();

    //     assert!(callback_invoked.get());
    // }

    #[test]
    fn test_add_preset() {
        let preset_model = test_model(); 

        assert_eq!(preset_model.row_count(), 2);
        preset_model.create_preset(frequency_preset3().into());
        assert_eq!(preset_model.row_count(), 3);

        assert_eq!(
            preset_model.row_data(2),
            Some(frequency_preset3())
        );
    }
}
