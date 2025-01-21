use std::rc::Rc;

use slint::Model;
use slint::ModelNotify;
use slint::ModelRc;
use slint::ModelTracker;

use crate::mvc;
// use crate::Callback;

#[derive(Clone)]
pub struct TaskListController {
    task_model: TaskModel,
    // show_create_task_callback: Rc<Callback<(), ()>>,
}

impl TaskListController {
    pub fn new(repo: impl mvc::traits::TaskRepository + 'static) -> Self {
        Self {
            task_model: TaskModel::new(repo),
            // show_create_task_callback: Rc::new(Callback::default()),
        }
    }

    pub fn task_model(&self) -> ModelRc<mvc::TaskStruct> {
        ModelRc::new(self.task_model.clone())
    }

    pub fn toggle_done(&self, index: usize) {
        self.task_model.toggle_done(index)
    }

    pub fn remove_task(&self, index: usize) {
        self.task_model.remove_task(index)
    }

    pub fn create_task(&self, title: &str, due_date: string) {
        self.task_model.push_task(mvc::TaskStruct {
            title: title.into(),
            due_date,
            ..Default::default()
        })
    }

    // pub fn show_create_task(&self) {
    //     self.show_create_task_callback.invoke(&());
    // }

    // pub fn on_show_create_task(&self, mut callback: impl FnMut() + 'static) {
    //     self.show_create_task_callback.on(move |()| {
    //         callback();
    //     });
    // }
}
