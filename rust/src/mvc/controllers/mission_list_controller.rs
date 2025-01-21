
use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::util;
use crate::mvc;
use crate::ui;
// use crate::mvc::MissionStruct;
// use crate::ui;
// use crate::mvc::FrequencyModel;
// use crate::mvc::PowerModel;
// use crate::Callback;

// #[derive(Clone)]
// pub struct MissionListController {
//     mission_model: MissionModel,
//     power_preset_repo: Rc<dyn mvc::traits::PowerPresetRepository + 'static>,
//     // show_create_mission_callback: Rc<Callback<(), ()>>,
//     // show_edit_mission_callback: Rc<Callback<(), ()>>,
// }

// impl MissionListController {
//     pub fn new(repo: impl mvc::traits::MissionRepository + 'static, power_preset_repo: Rc<dyn mvc::traits::PowerPresetRepository + 'static>) -> Self {
//         Self {
//             mission_model: MissionModel::new(repo),
//             power_preset_repo,
//             // show_create_mission_callback: Rc::new(Callback::default()),
//             // show_edit_mission_callback: Rc::new(Callback::default()),
//         }
//     }

//     // connects repo to a Slint `Model`` of Vec<MissionSlintStruct>
//     pub fn ui_mapping(&self) -> ModelRc<crate::ui::MissionSlintStruct> {
//         let connector: ModelRc<crate::ui::MissionSlintStruct> = Rc::new(slint::MapModel::new(self.mission_model(), mvc::MissionStruct::map_mission_to_slint)).into();
//         connector
//     }

//     pub fn mission_model(&self) -> ModelRc<mvc::MissionStruct> {
//         ModelRc::new(self.mission_model.clone())
//     }

//     // pub fn toggle_flagged(&self, index: usize) {
//     //     self.mission_model.toggle_flagged(index)
//     // }

//     pub fn get_mission(&self, index: usize) -> Option<mvc::MissionStruct> {
//         self.mission_model.get_mission(index)
//     }

//     pub fn remove_mission(&self, index: usize) {
//         self.mission_model.remove_mission(index)
//     }

//     pub fn update_mission(&self, index: usize, mission: ui::MissionSlintStruct) {
//         println!("controllers/frequency_mission_list_controller:create_mission");
//         self.mission_model.update_mission(index, mission.into())
//     }

//     pub fn create_mission(&self, mission: ui::MissionSlintStruct) {
//         let mission = mission.into();
//         self.mission_model.push_mission(mission)
//     }


//     // pub fn show_edit_mission(&self) {
//     //     println!("mission_list_controller.rs show_edit_mission_callback.invoke");
//     //     self.show_edit_mission_callback.invoke(&());
//     // }

//     // pub fn on_show_edit_mission(&self, mut callback: impl FnMut() + 'static) {
//     //     println!("mission_list_controller.rs on_show_edit_mission");
//     //     self.show_edit_mission_callback.on(move |()| {
//     //         callback();
//     //     });
//     // }    

//     // pub fn show_create_mission(&self) {
//     //     self.show_create_mission_callback.invoke(&());
//     // }

//     // pub fn on_show_create_mission(&self, mut callback: impl FnMut() + 'static) {
//     //     self.show_create_mission_callback.on(move |()| {
//     //         callback();
//     //     });
//     // }

// }

#[derive(Clone)]
pub struct MissionModel {
    repo: Rc<dyn mvc::traits::MissionRepository>,
    power_preset_repo: Rc<dyn mvc::traits::PowerPresetRepository>,
    notify: Rc<ModelNotify>,
}

impl MissionModel {
    pub fn new(repo: Rc<dyn mvc::traits::MissionRepository + 'static>, power_preset_repo: Rc<dyn mvc::traits::PowerPresetRepository + 'static>) -> Self {
        Self { 
            repo, 
            power_preset_repo, 
            notify: Rc::new(Default::default()) 
        }
    }

    // connects repo to a Slint `Model`` of Vec<MissionSlintStruct>
    pub fn ui_mapping(&self) -> ModelRc<crate::ui::MissionSlintStruct> {
        let wrapped_model: ModelRc<mvc::MissionStruct> = ModelRc::new(self.clone());
        let map_function = mvc::MissionStruct::map_mission_to_slint;
        let connector: ModelRc<crate::ui::MissionSlintStruct> = Rc::new(
            slint::MapModel::new(wrapped_model, map_function)
        ).into();
        connector       
    }

    // fn toggle_flagged(&self, index: usize) {
    //     if !self.repo.toggle_flagged(index) {
    //         return;
    //     }
    //     self.notify.row_changed(index)
    // }

    pub fn get_mission(&self, index: usize) -> Option<mvc::MissionStruct> {
        self.repo.get_mission(index)
    }
    pub fn update_mission(&self, index: usize, mission: ui::MissionSlintStruct) {
        if !self.repo.update_mission(index, mission.into()) {
            return;
        }
        self.notify.row_changed(index);
    }
    pub fn remove_mission(&self, index: usize) {
        if !self.repo.remove_mission(index) {
            return;
        }

        self.notify.row_removed(index, 1)
    }
    pub fn create_mission(&self, mission: ui::MissionSlintStruct) {
        if !self.repo.push_mission(mission.into()) {
            return;
        }

        self.notify.row_added(self.row_count() - 1, 1);
    }
    pub fn switch_power_preset(&self, mission: &mut ui::MissionSlintStruct, value: impl AsRef<str>) {
        // let power_preset_name = mission.power_model.preset_name.as_str();
        if value.as_ref() == "Custom" {
            return; // ignore special case
        }
        let power_preset_name = value.as_ref();
        match self.power_preset_repo.find_power_preset(power_preset_name) {
            Some(power_preset) => {
                println!("using power preset {}", power_preset_name);
                mission.power_model = power_preset.into();
            }
            None => {
                println!("did not find power preset {}", power_preset_name);
            }
        }
    }

    pub fn check_mission_field(&self, mut mission: ui::MissionSlintStruct, field_name: impl AsRef<str>, value: impl AsRef<str>) -> ui::MissionSlintStruct {
        match field_name.as_ref() {
            "mission_name" => mission.mission_name = value.as_ref().to_owned().into(),
            "mission_desc" => mission.mission_desc = value.as_ref().to_owned().into(),
            "power_preset_name" => self.switch_power_preset(&mut mission, value),
            "power1"|"power2"|"power3"|"power4"|"power5"|"power6"|"power7"|"power8" => {
                println!("changing {} to:{}", field_name.as_ref(), value.as_ref());
                match util::string_to_float(value.as_ref()) {
                    Some(f) => {
                        match field_name.as_ref() {
                            "power1" => mission.power_model.power1 = f,
                            "power2" => mission.power_model.power2 = f,
                            "power3" => mission.power_model.power3 = f,
                            "power4" => mission.power_model.power4 = f,
                            "power5" => mission.power_model.power5 = f,
                            "power6" => mission.power_model.power6 = f,
                            "power7" => mission.power_model.power7 = f,
                            "power8" => mission.power_model.power8 = f,
                            _ => {},
                        }
                        mission.power_model.preset_name = slint::SharedString::from("Custom");
                        mission.power_model.preset_desc = slint::SharedString::from("customized");
                        println!("power is now custom================================");
                        // self.custom_power(&mut mission, value)
                    }
                    None => {
                        println!("Invalid power value");
                    }
                }
            }
            _ => {
                println!("No check for field:{} value:{}", field_name.as_ref(), value.as_ref());
            }
        }
        mission
    }
}

impl Model for MissionModel {
    type Data = mvc::MissionStruct;

    fn row_count(&self) -> usize {
        self.repo.mission_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.repo.get_mission(row)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}


#[cfg(test)]
pub mod mission_tests {
    use super::*;
    use crate::mvc::{self, FrequencyStruct, FrequencyPresetStruct, PowerStruct, PowerPresetStruct};
    // use std::cell::Cell;

    pub fn mission1() -> mvc::MissionStruct {
        let power1 = PowerStruct{power1: 1.1, power2: 1.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1};
        let power_preset1 = PowerPresetStruct{power_preset_name: "power1".into(), power_preset_desc: "desc".into(), values: power1 };
        let frequency1 = FrequencyStruct{freq1: 1.1, freq2: 1.1, freq3: vec![1.1]};
        let frequency_preset1 = FrequencyPresetStruct{frequency_preset_name: "frequency1".into(), frequency_preset_desc: "desc".into(), values: frequency1 };
        let mission1 = mvc::MissionStruct { mission_name: "mission1".into(), mission_id: 1, mission_desc: "desc".into(), flagged: false, power_model: power_preset1, frequency_model: frequency_preset1  };
        mission1
    }
    pub fn mission2() -> mvc::MissionStruct {
        let power2 = PowerStruct{power1: 2.2, power2: 2.2, power3: 3.2, power4: 2.2, power5: 2.2, power6: 2.2, power7: 2.2, power8: 2.2, power9: 2, power10: 2};
        let power_preset2 = PowerPresetStruct{power_preset_name: "power2".into(), power_preset_desc: "desc".into(), values: power2 };
        let frequency2 = FrequencyStruct{freq1: 2.2, freq2: 2.2, freq3: vec![2.2]};
        let frequency_preset2 = FrequencyPresetStruct{frequency_preset_name: "frequency2".into(), frequency_preset_desc: "desc".into(), values: frequency2 };
        let mission2 = mvc::MissionStruct { mission_name: "mission2".into(), mission_id: 2, mission_desc: "desc".into(), flagged: false, power_model: power_preset2, frequency_model: frequency_preset2  };
        mission2
    }
    pub fn mission3() -> mvc::MissionStruct {
        let power3 = PowerStruct{power1: 3.3, power2: 3.3, power3: 3.3, power4: 3.3, power5: 3.3, power6: 3.3, power7: 3.3, power8: 3.3, power9: 3, power10: 3};
        let power_preset3 = PowerPresetStruct{power_preset_name: "power3".into(), power_preset_desc: "desc".into(), values: power3 };
        let frequency3 = FrequencyStruct{freq1: 3.3, freq2: 3.3, freq3: vec![3.3]};
        let frequency_preset3 = FrequencyPresetStruct{frequency_preset_name: "frequency3".into(), frequency_preset_desc: "desc".into(), values: frequency3 };
        let mission3 = mvc::MissionStruct { mission_name: "mission3".into(), mission_id: 3, mission_desc: "desc".into(), flagged: false, power_model: power_preset3, frequency_model: frequency_preset3  };
        mission3
    }
    pub fn power_preset1() -> PowerPresetStruct {
        let power1 = PowerStruct{power1: 1.1, power2: 1.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1};
        let power_preset1 = PowerPresetStruct{power_preset_name: "power1".into(), power_preset_desc: "desc".into(), values: power1 };
        power_preset1
    }
    pub fn power_preset2() -> PowerPresetStruct {
        let power2 = PowerStruct{power1: 2.2, power2: 2.2, power3: 3.2, power4: 2.2, power5: 2.2, power6: 2.2, power7: 2.2, power8: 2.2, power9: 2, power10: 2};
        let power_preset2 = PowerPresetStruct{power_preset_name: "power2".into(), power_preset_desc: "desc".into(), values: power2 };
        power_preset2
    }
    pub fn power_preset3() -> PowerPresetStruct {
        let power3 = PowerStruct{power1: 3.3, power2: 3.3, power3: 3.3, power4: 3.3, power5: 3.3, power6: 3.3, power7: 3.3, power8: 3.3, power9: 3, power10: 3};
        let power_preset3 = PowerPresetStruct{power_preset_name: "power3".into(), power_preset_desc: "desc".into(), values: power3 };
        power_preset3
    }
    pub fn power_preset_repo() -> mvc::MockPowerPresetRepository {
        mvc::MockPowerPresetRepository::new(vec![
            power_preset1(),
            power_preset2(),
            power_preset3(),
        ])
    }
    // pub fn test_controller() -> MissionListController {
    //     let missions = vec![mission1(), mission2() ];
    //     let power_preset_repo = Rc::new(power_preset_repo());
    //     let mission_repo = mvc::MockMissionRepository::new(missions);
    //     MissionListController::new(mission_repo, power_preset_repo)
    // }
    pub fn test_model() -> MissionModel {
        let missions = vec![mission1(), mission2() ];
        let power_preset_repo = power_preset_repo();
        let mission_repo = mvc::MockMissionRepository::new(missions);
        MissionModel::new(Rc::new(mission_repo), Rc::new(power_preset_repo))
    }

    #[test]
    fn test_missions() {
        // let controller = test_controller();
        // let mission_model = controller.mission_model();
        let mission_model = test_model();

        assert_eq!(mission_model.row_count(), 2);
        assert_eq!(
            mission_model.row_data(0),
            Some(mission1())
        );
        assert_eq!(
            mission_model.row_data(1),
            Some(mission2())
        );
    }

    // #[test]
    // fn test_toggle_mission_checked() {
    //     let controller = test_controller();
    //     let mission_model = controller.mission_model();

    //     assert!(mission_model.row_data(0).unwrap().flagged);
    //     controller.toggle_flagged(0);
    //     assert!(!mission_model.row_data(0).unwrap().flagged);
    // }

    #[test]
    fn test_remove_mission() {
        let mission_model = test_model();
        // let mission_model = controller.mission_model();
        // let power3 = PowerModel{power1: 1.1, power2: 1.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1};

        assert_eq!(mission_model.row_count(), 2);
        mission_model.remove_mission(0);
        assert_eq!(mission_model.row_count(), 1);

        assert_eq!(
            mission_model.row_data(0),
            Some(mission2())
        );
    }

    // #[test]
    // fn test_show_create_mission() {
    //     let controller = test_controller();

    //     let callback_invoked = Rc::new(Cell::new(false));

    //     controller.on_show_create_mission({
    //         let callback_invoked = callback_invoked.clone();

    //         move || {
    //             callback_invoked.set(true);
    //         }
    //     });

    //     controller.show_create_mission();

    //     assert!(callback_invoked.get());
    // }

    #[test]
    fn test_add_mission() {
        let mission_model = test_model();
        // let mission_model = controller.mission_model();
        // let power3 = PowerModel{power1: 1.1, power2: 1.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1};

        assert_eq!(mission_model.row_count(), 2);
        //controller.create_mission("mission3", 3, "desc".into(), false, PowerModel{power1: 1.0, power2: 2}, FrequencyModel{freq1: 1.0, freq2: 1.1, freq3: vec![2.0]});
        // controller.create_mission("mission3", 3, "desc".into(), false, "power1", "band1");
        mission_model.create_mission(mission3().into());
        assert_eq!(mission_model.row_count(), 3);

        assert_eq!(
            mission_model.row_data(2),
            Some(mission3())
        );
    }
}

