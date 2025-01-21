use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::util;
use crate::mvc;
use crate::ui;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct TaskStruct {
    pub title: String,
    pub priority: String,
    pub description: String,

    // due date in milliseconds
    pub due_date: String,
    pub done: bool,
}


#[derive(Clone)]
struct TaskModel {
    repo: Rc<dyn mvc::traits::TaskRepository>,
    notify: Rc<ModelNotify>,
}

impl TaskModel {
    pub fn new(repo: impl mvc::traits::TaskRepository + 'static) -> Self {
        Self { repo: Rc::new(repo), notify: Rc::new(Default::default()) }
    }

    pub fn toggle_done(&self, index: usize) {
        if !self.repo.toggle_done(index) {
            return;
        }

        self.notify.row_changed(index)
    }

    pub fn remove_task(&self, index: usize) {
        if !self.repo.remove_task(index) {
            return;
        }

        self.notify.row_removed(index, 1)
    }

    pub fn create_task(&self, task: mvc::TaskStruct) {
        if !self.repo.push_task(task) {
            return;
        }

        self.notify.row_added(self.row_count() - 1, 1);
    }
}

impl Model for TaskModel {
    type Data = mvc::TaskStruct;

    fn row_count(&self) -> usize {
        self.repo.task_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.repo.get_task(row)
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.notify.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mvc;
    // use std::cell::Cell;
    fn task1() -> TaskStruct {
        TaskStruct{title: "task1".into(), description: "desc".into(), priority: "low".into(), due_date: "1/1/1".into(), done: true, }
    }
    fn task2() -> TaskStruct {
        TaskStruct{title: "task2".into(), description: "desc".into(), priority: "low".into(), due_date: "1/1/1".into(), done: true, }
    }
    fn task3() -> TaskStruct {
        TaskStruct{title: "task3".into(), description: "desc".into(), priority: "low".into(), due_date: "1/1/1".into(), done: true, }
    }   

    fn test_model() -> TaskModel {
        TaskModel::new(mvc::MockTaskRepository::new(vec![ task1(), task2(), task3() ]))
    }

    #[test]
    fn test_tasks() {
        let task_model = test_model();

        assert_eq!(task_model.row_count(), 2);
        assert_eq!(
            task_model.row_data(0),
            Some(mvc::TaskStruct { title: "Item 1".into(), description: "desc".into(), due_date: "1/1/1".into(), done: true, priority: "low".into() },)
        );
        assert_eq!(
            task_model.row_data(1),
            Some(mvc::TaskStruct { title: "Item 2".into(), description: "desc".into(), due_date: "1/1/1".into(), done: false, priority: "low".into() },)
        );
    }

    #[test]
    fn test_toggle_task_checked() {
        let task_model = test_model();

        assert!(task_model.row_data(0).unwrap().done);
        task_model.toggle_done(0);
        assert!(!task_model.row_data(0).unwrap().done);
    }

    #[test]
    fn test_remove_task() {
        let task_model = test_model();

        assert_eq!(task_model.row_count(), 2);
        task_model.remove_task(0);
        assert_eq!(task_model.row_count(), 1);

        assert_eq!(
            task_model.row_data(0),
            Some(mvc::TaskStruct { title: "Item 2".into(), description: "desc".into(), due_date: "1/1/1".into(), done: false, priority: "low".into() },)
        );
    }

    #[test]
    fn test_add_task() {
        let task_model = test_model();

        assert_eq!(task_model.row_count(), 2);
        task_model.create_task(task3());
        assert_eq!(task_model.row_count(), 3);

        assert_eq!(
            task_model.row_data(2),
            Some(task3())
        );
    }
}
