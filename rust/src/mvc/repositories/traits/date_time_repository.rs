// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use crate::mvc;

pub trait DateTimeRepository {
    fn current_date(&self) -> mvc::DateStruct;
    fn current_time(&self) -> mvc::TimeStruct;
    fn date_to_string(&self, date: mvc::DateStruct) -> String;
    fn time_to_string(&self, time: mvc::TimeStruct) -> String;
    fn time_stamp(&self, date: mvc::DateStruct, time: mvc::TimeStruct) -> i32;
}
