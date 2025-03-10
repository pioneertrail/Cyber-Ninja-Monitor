use std::alloc::{alloc, dealloc, Layout};
use sysinfo::{System, SystemExt, CpuExt, NetworksExt};

#[test]
fn test_cpu_monitoring() {
    let mut sys = System::new();
    sys.refresh_cpu();
    
    // Test CPU usage calculation
    let cpu_list = sys.cpus();
    assert!(!cpu_list.is_empty());
    for cpu in cpu_list {
        assert!(cpu.cpu_usage() >= 0.0 && cpu.cpu_usage() <= 100.0);
    }
}

#[test]
fn test_memory_monitoring() {
    let mut sys = System::new();
    sys.refresh_memory();
    
    // Test memory usage calculation
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    assert!(used_memory <= total_memory);
    
    let used_memory_percentage = (used_memory as f32 / total_memory as f32) * 100.0;
    assert!(used_memory_percentage >= 0.0 && used_memory_percentage <= 100.0);
}

#[test]
fn test_system_info_accuracy() {
    let mut sys = System::new();
    sys.refresh_all();
    
    // Test system information retrieval
    assert!(sys.total_memory() > 0);
    assert!(sys.used_memory() <= sys.total_memory());
    
    // Test CPU information
    let cpu_list = sys.cpus();
    assert!(!cpu_list.is_empty());
    for cpu in cpu_list {
        assert!(cpu.cpu_usage() >= 0.0 && cpu.cpu_usage() <= 100.0);
    }
}

#[test]
fn test_memory_allocation() {
    unsafe {
        // Allocate some memory
        let layout = Layout::new::<[u8; 1024]>();
        let ptr = alloc(layout);
        assert!(!ptr.is_null());
        
        // Write some data
        ptr.write(42);
        assert_eq!(ptr.read(), 42);
        
        // Deallocate
        dealloc(ptr, layout);
    }
}

#[test]
fn test_network_monitoring() {
    let mut sys = System::new();
    sys.refresh_networks();
    
    // Just verify we can access network information without errors
    let networks = sys.networks();
    assert!(networks.iter().count() >= 0); // There might be no network interfaces
} 