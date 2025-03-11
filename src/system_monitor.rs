use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, NetworksExt};

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        SystemMonitor { sys }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    pub fn get_cpu_usage(&mut self) -> Vec<(String, f32)> {
        self.sys.refresh_cpu();
        self.sys.cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| (format!("CPU{}", i), cpu.cpu_usage()))
            .collect()
    }

    pub fn get_memory_usage(&mut self) -> (u64, u64, f32) {
        self.sys.refresh_memory();
        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        let usage = (used as f32 / total as f32) * 100.0;
        (total, used, usage)
    }

    pub fn get_disk_usage(&mut self) -> Vec<(String, u64, u64, f32)> {
        self.sys.refresh_disks();
        self.sys.disks()
            .iter()
            .map(|disk| {
                let mount_point = disk.mount_point().to_string_lossy().into_owned();
                let total = disk.total_space();
                let available = disk.available_space();
                let usage = ((total - available) as f32 / total as f32) * 100.0;
                (mount_point, total, available, usage)
            })
            .collect()
    }

    pub fn get_network_usage(&mut self) -> Vec<(String, u64, u64)> {
        self.sys.refresh_networks();
        self.sys.networks()
            .iter()
            .map(|(name, data)| {
                (name.clone(), data.received(), data.transmitted())
            })
            .collect()
    }

    pub fn get_system_info(&self) -> (String, String, String, String) {
        (
            self.sys.name().unwrap_or_default(),
            self.sys.kernel_version().unwrap_or_default(),
            self.sys.os_version().unwrap_or_default(),
            self.sys.host_name().unwrap_or_default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_usage() {
        let mut monitor = SystemMonitor::new();
        let cpu_usage = monitor.get_cpu_usage();
        for (_, usage) in &cpu_usage {
            assert!(*usage >= 0.0 && *usage <= 100.0, "CPU usage percentage must be between 0 and 100");
        }
    }

    #[test]
    fn test_memory_usage() {
        let mut monitor = SystemMonitor::new();
        let (total, used, usage) = monitor.get_memory_usage();
        assert!(usage >= 0.0 && usage <= 100.0, "Memory usage percentage must be between 0 and 100");
        assert!(used <= total, "Used memory cannot exceed total memory");
    }

    #[test]
    fn test_disk_usage() {
        let mut monitor = SystemMonitor::new();
        let disk_info = monitor.get_disk_usage();
        for (_, total, available, _) in &disk_info {
            assert!(available <= total, "Available space cannot exceed total space");
        }
    }

    #[test]
    fn test_network_usage() {
        let mut monitor = SystemMonitor::new();
        let network_info = monitor.get_network_usage();
        for (_, received, transmitted) in &network_info {
            assert!(*received >= 0, "Received bytes cannot be negative");
            assert!(*transmitted >= 0, "Transmitted bytes cannot be negative");
        }
    }

    #[test]
    fn test_system_info_validity() {
        let monitor = SystemMonitor::new();
        let (name, kernel, os_version, hostname) = monitor.get_system_info();
        
        // Basic validity checks
        assert!(!name.is_empty(), "OS name should not be empty");
        assert!(!kernel.is_empty(), "Kernel version should not be empty");
        assert!(!os_version.is_empty(), "OS version should not be empty");
        assert!(!hostname.is_empty(), "Hostname should not be empty");
        
        // Format checks - hostname should not contain path separators
        assert!(!hostname.contains('/') && !hostname.contains('\\'), 
            "Hostname should not contain path separators");
    }
} 