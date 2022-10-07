// Setup needed files and verify file integrity


use winreg::enums::*;
use winreg::RegKey;
use sysinfo::*;
use sqlite;
use crate::winapi_backend::services_manager::*;
use std::fmt;

pub struct ComputerInfo {
    computer_name: String,
    domain: String,
    product_version: String,
    os_version: String,
    execution_policy: String,
    firewall_enabled: i32
}

impl fmt::Display for ComputerInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "computername: {}, domain: {}, product_version: {}, os_version: {}, exexcution_policy: {}, firewall_enabled: {}", self.computer_name, self.domain, self.product_version, self.os_version, self.execution_policy, self.firewall_enabled)
    }
}



pub fn overall_info() -> ComputerInfo {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion").unwrap();
    let active_computer_name = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\ComputerName\\ActiveComputerName").unwrap();
    let powershell = hklm.open_subkey("SOFTWARE\\Microsoft\\PowerShell\\1\\ShellIds\\Microsoft.PowerShell").unwrap();
    let tcpip_params = hklm.open_subkey("SYSTEM\\CurrentControlSet\\Services\\Tcpip\\Parameters").unwrap();
    let domain: String = tcpip_params.get_value("Domain").unwrap();
    let firewall_enabled = get_service_state("Dhcp");
    

    let domain = if domain.is_empty() == true {
        "System is not in a domain".to_string()
    } else {
        domain
    };

    let display_version: String = cur_ver.get_value("DisplayVersion").unwrap();
    let release_id: String = cur_ver.get_value("ReleaseId").unwrap();

    let info = ComputerInfo {
        computer_name: active_computer_name.get_value("ComputerName").unwrap(),
        domain: domain,
        product_version: cur_ver.get_value("ProductName").unwrap(),
        os_version: format!("{} {}", display_version, release_id),
        execution_policy: powershell.get_value("ExecutionPolicy").unwrap(),
        firewall_enabled: firewall_enabled
    };
    println!("{}", firewall_enabled);
    return info
}

