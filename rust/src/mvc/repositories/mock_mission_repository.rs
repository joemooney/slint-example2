// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use std::{cell::RefCell, rc::Rc};

use super::traits;
use crate::mvc;

#[derive(Clone)]
pub struct MockMissionRepository {
    missions: Rc<RefCell<Vec<mvc::MissionStruct>>>,
}

impl MockMissionRepository {
    pub fn new(missions: Vec<mvc::MissionStruct>) -> Self {
        Self { missions: Rc::new(RefCell::new(missions)) }
    }
}

impl traits::MissionRepository for MockMissionRepository {
    fn mission_count(&self) -> usize {
        self.missions.borrow().len()
    }

    fn get_mission(&self, index: usize) -> Option<mvc::MissionStruct> {
        self.missions.borrow().get(index).cloned()
    }

    // fn toggle_flagged(&self, index: usize) -> bool {
    //     if let Some(mission) = self.missions.borrow_mut().get_mut(index) {
    //         mission.flagged = !mission.flagged;
    //         return true;
    //     }

    //     false
    // }

    fn remove_mission(&self, index: usize) -> bool {
        if index < self.missions.borrow().len() {
            self.missions.borrow_mut().remove(index);
            return true;
        }

        false
    }

    fn push_mission(&self, mission: mvc::MissionStruct) -> bool {
        self.missions.borrow_mut().push(mission);
        true
    }
 
    fn update_mission(&self, index: usize, mission: mvc::MissionStruct) -> bool {
        if index < self.missions.borrow().len() {
            self.missions.borrow_mut()[index] = mission;
            return true
        }
        false
    }   
}
