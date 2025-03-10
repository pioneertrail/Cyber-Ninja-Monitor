use sysinfo::{System, SystemExt, CpuExt, DiskExt};

// Module declarations
pub mod system_monitor;
pub mod network_stats;
pub mod ai_personality;
pub mod tts;
pub mod theme;
pub mod audio_manager;
pub mod personality_modal;

// Re-export public types
pub use system_monitor::SystemMonitor;
pub use network_stats::NetworkStats;
pub use ai_personality::AIPersonality;
pub use tts::TTSManager;
pub use theme::CyberTheme;
pub use audio_manager::AudioManager;
pub use personality_modal::PersonalityModal;

// Constants
pub const MIN_MEMORY_GB: f64 = 4.0;
pub const MIN_CPU_CORES: usize = 2;
pub const MIN_DISK_GB: f64 = 10.0;

// Helper functions
pub fn get_system_info() -> System {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys
}

pub fn get_total_memory_gb(sys: &System) -> f64 {
    (sys.total_memory() as f64) / (1024.0 * 1024.0 * 1024.0)
}

pub fn get_cpu_cores(sys: &System) -> usize {
    sys.cpus().len()
}

pub fn get_available_disk_space_gb(sys: &System) -> f64 {
    let mut total_available = 0.0;
    for disk in sys.disks() {
        total_available += (disk.available_space() as f64) / (1024.0 * 1024.0 * 1024.0);
    }
    total_available
}

pub fn check_system_requirements(sys: &System) -> bool {
    let memory_gb = get_total_memory_gb(sys);
    let cpu_cores = get_cpu_cores(sys);
    let disk_space_gb = get_available_disk_space_gb(sys);

    memory_gb >= MIN_MEMORY_GB && 
    cpu_cores >= MIN_CPU_CORES && 
    disk_space_gb >= MIN_DISK_GB
} 