use gix::progress::{Id, MessageLevel, Progress, Step, StepShared, Unit};
use gix::Count;
use std::sync::atomic::Ordering;

pub struct FuncProgress {
    name: String,
    id: Id,
    step: StepShared,
    max: Option<Step>,
    unit: Option<Unit>,
}

impl FuncProgress {
    pub fn new(name: impl Into<String>, id: Id) -> Self {
        FuncProgress {
            name: name.into(),
            id,
            step: Default::default(),
            max: None,
            unit: None,
        }
    }
}

impl Count for FuncProgress {
    fn set(&self, step: Step) {
        self.step.store(step, Ordering::SeqCst);
    }

    fn step(&self) -> Step {
        self.step.load(Ordering::SeqCst)
    }

    fn inc_by(&self, step: Step) {
        self.step.fetch_add(step, Ordering::SeqCst);
    }

    fn counter(&self) -> StepShared {
        self.step.clone()
    }
}

impl Progress for FuncProgress {
    fn init(&mut self, max: Option<Step>, unit: Option<Unit>) {
        self.max = max;
        self.unit = unit;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn name(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn id(&self) -> Id {
        self.id
    }

    fn message(&self, level: MessageLevel, message: String) {
        match level {
            MessageLevel::Info => println!("ℹ{} → {}", self.name, message),
            MessageLevel::Failure => println!("𐄂{} → {}", self.name, message),
            MessageLevel::Success => println!("✓{} → {}", self.name, message),
        }
    }
}
