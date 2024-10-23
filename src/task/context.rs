use anyhow::Context;
use esp_idf_hal::{
    gpio::{AnyIOPin, Pin},
    peripheral::PeripheralRef,
};

use super::{priority::TaskPriority, status::TaskStatus};

pub struct TaskContext<'a> {
    name: String,
    id: uuid::Uuid,
    priority: TaskPriority,
    state: TaskStatus,
    program_counter: usize,
    pins_used: Vec<PeripheralRef<'a, AnyIOPin>>, // TODO: Add support for storing:
                                                 // - Memory Pointers (To the program code and data, probably not needed)
                                                 // - Context Data (Data present in registers, used for saving state when restarting a process from middle)
                                                 // - IO Status Info (List of IO requests, devices assigned to it, list of files used, etc.)
                                                 // - Accounting Info (Processor time used so far, clock time used, time limits before its aborted, etc.)
}

impl<'a> TaskContext<'a> {
    pub fn new(name: String, priority: TaskPriority) -> Self {
        Self {
            name,
            id: uuid::Uuid::new_v4(),
            priority,
            state: TaskStatus::New,
            program_counter: 0,
            pins_used: vec![],
        }
    }

    pub fn pins(&mut self) -> &mut Vec<PeripheralRef<'a, AnyIOPin>> {
        &mut self.pins_used
    }

    pub fn find_pin(
        &mut self,
        number: i32,
    ) -> Result<&mut PeripheralRef<'a, AnyIOPin>, anyhow::Error> {
        self.pins()
            .iter_mut()
            .find(|p| p.pin() == number)
            .context("Use of resource pin not specified.")
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn priority(&self) -> TaskPriority {
        self.priority
    }

    pub fn state(&self) -> TaskStatus {
        self.state
    }

    pub fn program_counter(&self) -> usize {
        self.program_counter
    }
}
