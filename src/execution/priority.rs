#[derive(Debug, Clone, Copy)]
pub enum PriorityLevel {
    Low,
    Medium,
    High,
}

impl PriorityLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            PriorityLevel::Low => "low",
            PriorityLevel::Medium => "medium",
            PriorityLevel::High => "high",
        }
    }
}
