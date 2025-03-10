use sysinfo::{NetworkExt, System, SystemExt};

pub struct NetworkStats {
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
    pub total_networks: usize,
}

impl NetworkStats {
    pub fn new(sys: &System) -> Self {
        let networks = sys.networks();
        let mut received = 0;
        let mut transmitted = 0;
        let mut network_count = 0;

        for (_interface_name, network) in networks {
            received += network.received();
            transmitted += network.transmitted();
            network_count += 1;
        }

        NetworkStats {
            received_bytes: received,
            transmitted_bytes: transmitted,
            total_networks: network_count,
        }
    }

    pub fn update(&mut self, sys: &System) {
        let networks = sys.networks();
        self.received_bytes = 0;
        self.transmitted_bytes = 0;
        let mut network_count = 0;

        for (_interface_name, network) in networks {
            self.received_bytes += network.received();
            self.transmitted_bytes += network.transmitted();
            network_count += 1;
        }
        self.total_networks = network_count;
    }
} 