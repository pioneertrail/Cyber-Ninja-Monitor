use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt, ProcessExt, NetworksExt, PidExt};
use std::thread;
use std::time::Duration;

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        // Add a small delay to allow CPU usage to be measured
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.sys.refresh_cpu();
    }

    pub fn get_memory_info(&self) -> (u64, u64, f32) {
        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        let usage = (used as f32 / total as f32) * 100.0;
        (total, used, usage)
    }

    pub fn get_cpu_usage(&self) -> Vec<(String, f32)> {
        self.sys.cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| {
                let name = format!("Core {}", i);
                (name, cpu.cpu_usage())
            })
            .collect()
    }

    pub fn get_disk_info(&self) -> Vec<(String, u64, u64)> {
        self.sys.disks()
            .iter()
            .map(|disk| {
                (
                    disk.mount_point().to_string_lossy().to_string(),
                    disk.total_space(),
                    disk.available_space(),
                )
            })
            .collect()
    }

    pub fn get_network_info(&self) -> Vec<(String, u64, u64)> {
        self.sys.networks()
            .iter()
            .map(|(name, data)| {
                (
                    name.to_string(),
                    data.received(),
                    data.transmitted(),
                )
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

    pub fn get_process_info(&self) -> Vec<(u32, String, f32)> {
        self.sys.processes()
            .iter()
            .map(|(pid, process)| {
                (pid.as_u32(), process.name().to_string(), process.cpu_usage())
            })
            .collect()
    }

    /// Continuously monitors system resources and prints updates to the console.
    /// This method runs in an infinite loop and is designed for command-line usage.
    /// It provides real-time updates on:
    /// - CPU usage per core
    /// - Memory usage and availability
    /// - Disk space and I/O
    /// - Network interface statistics
    /// - Top processes by CPU usage
    ///
    /// # Example
    /// ```no_run
    /// use cyber_ninja_monitor::SystemMonitor;
    /// 
    /// let mut monitor = SystemMonitor::new();
    /// // This will run indefinitely until interrupted
    /// monitor.monitor_continuously();
    /// ```
    ///
    /// Note: This method is primarily used for debugging and system analysis.
    /// For GUI applications, use the individual monitoring methods instead.
    pub fn monitor_continuously(&mut self) {
        loop {
            self.refresh();
            
            // Print system info
            let (name, kernel, os_version, hostname) = self.get_system_info();
            println!("\nSystem Information:");
            println!("Name: {}", name);
            println!("Kernel: {}", kernel);
            println!("OS Version: {}", os_version);
            println!("Hostname: {}", hostname);

            // Print memory usage
            let (total, used, usage) = self.get_memory_info();
            println!("\nMemory Usage:");
            println!("Total: {} MB", total / 1024 / 1024);
            println!("Used: {} MB", used / 1024 / 1024);
            println!("Usage: {:.1}%", usage);

            // Print CPU usage
            println!("\nCPU Usage:");
            for (name, usage) in self.get_cpu_usage() {
                println!("{}: {:.1}%", name, usage);
            }

            // Print disk info
            println!("\nDisk Information:");
            for (mount_point, total, available) in self.get_disk_info() {
                println!("Mount point: {}", mount_point);
                println!("Total: {} GB", total / 1024 / 1024 / 1024);
                println!("Available: {} GB", available / 1024 / 1024 / 1024);
            }

            // Print network info
            println!("\nNetwork Information:");
            for (interface, rx, tx) in self.get_network_info() {
                println!("Interface: {}", interface);
                println!("Received: {} KB", rx / 1024);
                println!("Transmitted: {} KB", tx / 1024);
            }

            // Print top processes by CPU usage
            println!("\nTop Processes by CPU Usage:");
            let mut processes = self.get_process_info();
            processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
            for (pid, name, cpu_usage) in processes.iter().take(5) {
                println!("[{}] {}: {:.1}%", pid, name, cpu_usage);
            }

            thread::sleep(Duration::from_secs(2));
            print!("\x1B[2J\x1B[1;1H"); // Clear screen
        }
    }

    pub fn is_initialized(&self) -> bool {
        true // The monitor is always initialized when created
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

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

    #[test]
    fn test_cpu_usage_validity() {
        let mut monitor = SystemMonitor::new();
        
        // Initial reading
        let cpu_info = monitor.get_cpu_usage();
        
        for (name, usage) in &cpu_info {
            // Name format check - should start with either "CPU " or "Core "
            assert!(name.starts_with("CPU ") || name.starts_with("Core "), 
                "CPU name format invalid: {}", name);
            
            // Usage bounds check
            assert!(usage >= &0.0 && usage <= &100.0, 
                "CPU usage out of bounds: {} = {}%", name, usage);
        }
        
        // Check for reasonable changes over time
        thread::sleep(Duration::from_millis(100));
        let second_reading = monitor.get_cpu_usage();
        
        for (name, usage) in &second_reading {
            if let Some(prev_usage) = cpu_info.iter().find(|(n, _)| n == name).map(|(_, u)| u) {
                assert!((usage - prev_usage).abs() < 100.0, 
                    "CPU usage showing impossible jump: {} -> {}", prev_usage, usage);
            }
        }
    }

    #[test]
    fn test_memory_info_validity() {
        let monitor = SystemMonitor::new();
        let (total, used, usage) = monitor.get_memory_info();
        
        // Basic bounds checking
        assert!(total > 0, "Total memory should be greater than 0");
        assert!(used <= total, "Used memory cannot exceed total memory");
        assert!(usage >= 0.0 && usage <= 100.0, "Memory usage percentage must be between 0 and 100");
        
        // Alignment check (most systems use 4KB pages)
        const PAGE_SIZE: u64 = 4096;
        assert!(total % PAGE_SIZE == 0, "Total memory should be page-aligned");
        
        // Reasonable minimum memory check (most modern systems have at least 1GB)
        const MIN_MEMORY: u64 = 1024 * 1024 * 1024; // 1GB
        assert!(total >= MIN_MEMORY, "System should have at least 1GB memory");
    }

    #[test]
    fn test_disk_info_validity() {
        let monitor = SystemMonitor::new();
        let disk_info = monitor.get_disk_info();
        
        for (mount_point, total, available) in &disk_info {
            // Mount point validity
            assert!(!mount_point.is_empty(), "Mount point should not be empty");
            
            // Space checks
            assert!(*total > 0, "Total disk space should be greater than 0");
            assert!(*available <= *total, "Available space cannot exceed total space");
            
            // Block size alignment (typically 512 bytes)
            assert!(total % 512 == 0, "Disk space should be block-aligned");
            assert!(available % 512 == 0, "Available space should be block-aligned");
            
            // Usage sanity check
            let usage_percent = 100.0 * (1.0 - (*available as f64 / *total as f64));
            assert!(usage_percent <= 100.0, "Disk usage percentage cannot exceed 100%");
        }
    }

    #[test]
    fn test_network_info_validity() {
        let monitor = SystemMonitor::new();
        let network_info = monitor.get_network_info();
        
        for (interface, rx, tx) in &network_info {
            // Interface name check
            assert!(!interface.is_empty(), "Network interface name should not be empty");
            
            // Traffic validity
            assert!(*rx >= 0, "Received bytes cannot be negative");
            assert!(*tx >= 0, "Transmitted bytes cannot be negative");
        }
    }
} 