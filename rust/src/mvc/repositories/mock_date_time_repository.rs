// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use crate::mvc;

use super::traits;

#[derive(Clone)]
pub struct MockDateTimeRepository {
    current_date: mvc::DateStruct,
    current_time: mvc::TimeStruct,
    time_stamp: i32,
}

impl MockDateTimeRepository {
    pub fn new(
        current_date: mvc::DateStruct,
        current_time: mvc::TimeStruct,
        time_stamp: i32,
    ) -> Self {
        Self { current_date, current_time, time_stamp }
    }
}

impl traits::DateTimeRepository for MockDateTimeRepository {
    fn current_date(&self) -> mvc::DateStruct {
        self.current_date
    }

    fn current_time(&self) -> mvc::TimeStruct {
        self.current_time
    }

    fn date_to_string(&self, date: mvc::DateStruct) -> String {
        format!("{}/{}/{}", date.year, date.month, date.day)
    }

    fn time_to_string(&self, time: mvc::TimeStruct) -> String {
        format!("{}:{}", time.hour, time.minute)
    }

    fn time_stamp(&self, _date: mvc::DateStruct, _time: mvc::TimeStruct) -> i32 {
        self.time_stamp
    }
}
