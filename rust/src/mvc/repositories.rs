mod mock_date_time_repository;
// use std::collections::HashMap;

pub use mock_date_time_repository::*;

mod mock_task_repository;
pub use mock_task_repository::*;

mod mock_mission_repository;
pub use mock_mission_repository::*;

mod mock_power_preset_repository;
pub use mock_power_preset_repository::*;

mod mock_frequency_preset_repository;
pub use mock_frequency_preset_repository::*;

use crate::mvc::models::{DateStruct, TaskStruct, MissionStruct, PowerPresetStruct, PowerStruct, FrequencyPresetStruct, FrequencyStruct, TimeStruct};

pub mod traits;

pub fn date_time_repo() -> impl traits::DateTimeRepository + Clone {
    MockDateTimeRepository::new(
        DateStruct { year: 2025, month: 1, day: 1 },
        TimeStruct { hour: 16, minute: 43, second: 0 },
        1718183634,
    )
}

pub fn task_repo() -> impl traits::TaskRepository + Clone {
    MockTaskRepository::new(vec![
        TaskStruct { title: "Power on".into(), done: false, due_date: 1717986537151, priority: "low".into() },
        TaskStruct { title: "Talk".into(), done: false, due_date: 1717986537151, priority: "med".into() },
        TaskStruct {
            title: "Power off".into(),
            done: false, 
            priority: "high".into(),
            due_date: 1717986537151,
        },
    ])
}

pub fn mission_repo() -> impl traits::MissionRepository + Clone {
    // let power1 = PowerPresetStruct { preset_name: "power1".into(), preset_desc: "desc".into(), values: PowerStruct{power1: 1.0, power2: 2} };
    // let power2 = PowerPresetStruct { preset_name: "power2".into(), preset_desc: "desc".into(), values: PowerStruct{power1: 1.0, power2: 2} };
    // let frequency1 = FrequencyPresetStruct { preset_name: "frequency1".into(), preset_desc: "desc".into(), values: FrequencyStruct{freq1: 1.0, freq2: vec![2.0]} };
    // let frequency2 = FrequencyPresetStruct { preset_name: "frequency2".into(), preset_desc: "desc".into(), values: FrequencyStruct{freq1: 1.0, freq2: vec![2.0]} };
    let power1 = PowerStruct{power1: 1.1, power2: 1.1, power3: 3.1, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1};
    let power2 = PowerStruct{power1: 2.2, power2: 2.1, power3: 3.1, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1};
    let power_preset1 = PowerPresetStruct{power_preset_name: "power1".into(), power_preset_desc: "desc".into(), values: power1 };
    let power_preset2 = PowerPresetStruct{power_preset_name: "power2".into(), power_preset_desc: "desc".into(), values: power2 };
    let frequency1 = FrequencyStruct{freq1: 1.1, freq2: 1.1, freq3: vec![1.1]};
    let frequency2 = FrequencyStruct{freq1: 2.2, freq2: 1.1, freq3: vec![2.2]};
    let frequency_preset1 = FrequencyPresetStruct{frequency_preset_name: "frequency1".into(), frequency_preset_desc: "desc".into(), values: frequency1 };
    let frequency_preset2 = FrequencyPresetStruct{frequency_preset_name: "frequency2".into(), frequency_preset_desc: "desc".into(), values: frequency2 };
    MockMissionRepository::new(vec![
        MissionStruct { mission_name: "mission1".into(), mission_id: 1, mission_desc: "desc".into(), flagged: false, power_model: power_preset1, frequency_model: frequency_preset1  },
        MissionStruct { mission_name: "mission2".into(), mission_id: 2, mission_desc: "desc".into(), flagged: false, power_model: power_preset2, frequency_model: frequency_preset2  },
    ])
}

pub fn power_preset_repo() -> impl traits::PowerPresetRepository + Clone {
    // MockPowerPresetRepository::new(HashMap::from([
    //     ("power1".to_owned(), PowerPresetStruct { preset_id:0, preset_name: "power1".into(), preset_desc: "desc".into(), values: PowerStruct{power1: 1.1, power2: 1.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1} }),
    //     ("power2".to_owned(), PowerPresetStruct { preset_id:1, preset_name: "power2".into(), preset_desc: "desc".into(), values: PowerStruct{power1: 2.2, power2: 2.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1} }),
    // ]))
    MockPowerPresetRepository::new(vec![
        PowerPresetStruct { power_preset_name: "power1".into(), power_preset_desc: "desc".into(), values: PowerStruct{power1: 1.1, power2: 1.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1} },
        PowerPresetStruct { power_preset_name: "power2".into(), power_preset_desc: "desc".into(), values: PowerStruct{power1: 2.2, power2: 2.2, power3: 3.2, power4: 1.1, power5: 1.1, power6: 1.1, power7: 1.1, power8: 1.1, power9: 1, power10: 1} },
    ])
}

pub fn frequency_preset_repo() -> impl traits::FrequencyPresetRepository + Clone {
    // MockFrequencyPresetRepository::new(HashMap::from([
    //     ("frequency1".to_owned(), FrequencyPresetStruct { preset_id:1, preset_name: "frequency1".into(), preset_desc: "desc".into(), values: FrequencyStruct{freq1: 1.1, freq2: 1.1, freq3: vec![1.1]} }),
    //     ("frequency2".to_owned(), FrequencyPresetStruct { preset_id:2, preset_name: "frequency2".into(), preset_desc: "desc".into(), values: FrequencyStruct{freq1: 2.2, freq2: 1.1, freq3: vec![2.2]} }),
    // ]))
    MockFrequencyPresetRepository::new(vec![
        FrequencyPresetStruct { frequency_preset_name: "frequency1".into(), frequency_preset_desc: "desc".into(), values: FrequencyStruct{freq1: 1.1, freq2: 1.1, freq3: vec![1.1]} },
        FrequencyPresetStruct { frequency_preset_name: "frequency2".into(), frequency_preset_desc: "desc".into(), values: FrequencyStruct{freq1: 2.2, freq2: 1.1, freq3: vec![2.2]} },
    ])
}
