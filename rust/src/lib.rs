// use slint::platform::WindowEvent;
// #[cfg(target_arch = "wasm32")]
// use wasm_bindgen::prelude::*;

pub mod mvc;
pub mod util;
pub mod ui;

// mod callback;
// pub use callback::*;
pub use slint::*;

// #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
// slint::include_modules!();

fn init() -> ui::MainWindow {

    // let mission_model = Rc::new(slint::VecModel::from())
    let main_window = ui::MainWindow::new().unwrap();
    //--------------------------------------------------------------------------------------------
    let mission_repo = mvc::mission_repo();
    let mission_controller = mvc::MissionListController::new(mission_repo);
    main_window.set_missions(mission_controller.connector());

    let ctrl = mission_controller.clone();
    main_window.on_s2r_create_mission(move |mission|{
        println!("create mission {:#?}", mission);
        ctrl.create_mission(mission);
    });

    let ctrl = mission_controller.clone();
    main_window.on_s2r_update_mission(move |index, mission|{
        println!("update mission {:#?}", mission);
        ctrl.update_mission(index as usize, mission);
    });

    let ctrl = mission_controller.clone();
    main_window.on_s2r_check_mission_field(move |mission, field_name, value|{
        println!("check mission field {:#?}", mission);
        ctrl.check_mission_field(mission, field_name, value)
    });
    //--------------------------------------------------------------------------------------------
    let power_preset_repo = mvc::power_preset_repo();
    let power_preset_controller = mvc::PowerPresetListController::new(power_preset_repo);
    main_window.set_power_presets(power_preset_controller.connector());

    let ctrl = power_preset_controller.clone();
    main_window.on_s2r_create_power_preset(move |power_preset|{
        println!("create power_preset {:#?}", power_preset);
        ctrl.create_preset(power_preset);
    });

    let ctrl = power_preset_controller.clone();
    main_window.on_s2r_update_power_preset(move |index, power_preset|{
        println!("update power_preset {:#?}", power_preset);
        ctrl.update_preset(index as usize, power_preset);
    });

    let ctrl = power_preset_controller.clone();
    main_window.on_s2r_check_power_preset_field(move |power_preset, field_name, value|{
        println!("check power_preset field {:#?}", power_preset);
        ctrl.check_preset_field(power_preset, field_name, value)
    });
    //--------------------------------------------------------------------------------------------

    main_window
}

pub fn main() {
    let main_window = init();
    main_window.run().unwrap();
}

// fn init() -> ui::MainWindow {
//     let view_handle = ui::MainWindow::new().unwrap();
//     // let app = view_handle.as_weak();
//     // let window = app.window();

//     let task_list_controller = mvc::TaskListController::new(mvc::task_repo());
//     ui::task_list_adapter::connect(&view_handle, task_list_controller.clone());
//     ui::navigation_adapter::connect_task_list_controller(
//         &view_handle,
//         task_list_controller.clone(),
//     );

//     let create_task_controller = mvc::CreateTaskController::new(mvc::date_time_repo());
//     ui::create_task_adapter::connect(&view_handle, create_task_controller.clone());
//     ui::navigation_adapter::connect_create_task_controller(&view_handle, create_task_controller);
//     ui::create_task_adapter::connect_task_list_controller(&view_handle, task_list_controller);

//     let mission_list_controller = mvc::MissionListController::new(mvc::mission_repo());
//     ui::mission_list_adapter::connect(&view_handle, mission_list_controller.clone());
//     ui::navigation_adapter::connect_mission_list_controller(
//         &view_handle,
//         mission_list_controller.clone(),
//     );

//     // let create_mission_controller = mvc::CreateMissionController::new(mvc::date_time_repo());
//     let create_mission_controller = mvc::CreateMissionController::new();
//     ui::create_mission_adapter::connect(&view_handle, create_mission_controller.clone());
//     ui::navigation_adapter::connect_create_mission_controller(&view_handle, create_mission_controller);
//     ui::create_mission_adapter::connect_mission_list_controller(&view_handle, mission_list_controller);

//     let power_preset_list_controller = mvc::PowerPresetListController::new(mvc::power_preset_repo());
//     ui::power_preset_list_adapter::connect(&view_handle, power_preset_list_controller.clone());
//     ui::navigation_adapter::connect_power_preset_list_controller(
//         &view_handle,
//         power_preset_list_controller.clone(),
//     );

//     let create_power_preset_controller = mvc::CreatePowerPresetController::new();
//     ui::create_power_preset_adapter::connect(&view_handle, create_power_preset_controller.clone());
//     ui::navigation_adapter::connect_create_power_preset_controller(&view_handle, create_power_preset_controller);
//     ui::create_power_preset_adapter::connect_power_preset_list_controller(&view_handle, power_preset_list_controller);

//     let frequency_preset_list_controller = mvc::FrequencyPresetListController::new(mvc::frequency_preset_repo());
//     ui::frequency_preset_list_adapter::connect(&view_handle, frequency_preset_list_controller.clone());
//     ui::navigation_adapter::connect_frequency_preset_list_controller(
//         &view_handle,
//         frequency_preset_list_controller.clone(),
//     );

//     let create_frequency_preset_controller = mvc::CreateFrequencyPresetController::new();
//     ui::create_frequency_preset_adapter::connect(&view_handle, create_frequency_preset_controller.clone());
//     ui::navigation_adapter::connect_create_frequency_preset_controller(&view_handle, create_frequency_preset_controller);
//     ui::create_frequency_preset_adapter::connect_frequency_preset_list_controller(&view_handle, frequency_preset_list_controller);

//     // // Font size state
//     // let mut font_size = 16;

//     // // Handle keyboard shortcuts for font size adjustment
//     // view_handle.window().on_event(move |event| {
//     //     if let WindowEvent::KeyPressed { text, modifiers } = event {
//     //         if modifiers.control { // Check if Control key is pressed
//     //             match text.as_str() {
//     //                 "+" => {
//     //                     font_size += 1;
//     //                     app.set_font_size(font_size);
//     //                     println!("Increased font size to: {}", font_size);
//     //                 }
//     //                 "-" => {
//     //                     if font_size > 1 {
//     //                         font_size -= 1;
//     //                         app.set_font_size(font_size);
//     //                         println!("Decreased font size to: {}", font_size);
//     //                     }
//     //                 }
//     //                 _ => {}
//     //             }
//     //         }
//     //     }
//     // });

//     view_handle
// }