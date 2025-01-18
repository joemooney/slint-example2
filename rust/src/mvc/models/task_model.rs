// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TaskStruct {
    pub title: String,
    pub priority: String,

    // due date in milliseconds
    pub due_date: i64,
    pub done: bool,
}
