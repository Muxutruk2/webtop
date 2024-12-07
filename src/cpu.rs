use sysinfo::{System};
pub struct CPUInfo {
    pub model_name: Option<String>,
}

impl CPUInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let name = sys.cpus().first().map(|cpu| cpu.brand().to_string());
        CPUInfo { model_name: name }
    }
    pub fn get_brand_name(&self) -> Option<String> {
        self.model_name.clone()
    }
}
