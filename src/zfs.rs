use crate::{block_device, utils};
use subprocess::Exec;

pub fn import_pool(device: &block_device::BlockDevice, mount_point: &str) -> bool {
    let pool_name = device.zpool_name.as_ref().unwrap_or(&"zpcachyos".to_string());

    log::info!("Importing ZFS pool '{}' from partition {}", pool_name, device.name);
    
    let result = Exec::cmd("zpool")
        .args(&["import", "-f", "-R", mount_point, pool_name])
        .join();
        
    if result.is_err() || !result.unwrap().success() {
        utils::print_error_and_exit(&format!(
            "Failed to import ZFS pool '{}' from partition {}",
            pool_name, device.name
        ));
    }

    // Try CachyOS default root dataset first
    let cachyos_root = format!("{}/ROOT/cos/root", pool_name);
    log::info!("Mounting CachyOS root dataset '{}'", cachyos_root);
    
    let mount_result = Exec::cmd("zfs")
        .args(&["mount", &cachyos_root])
        .join();
        
    if mount_result.is_err() || !mount_result.unwrap().success() {
        utils::print_error_and_exit(&format!(
            "Failed to mount CachyOS root dataset '{}'",
            cachyos_root
        ));
    }

    true
}

pub fn mount_all_datasets(pool_name: &str) -> bool {
    log::info!("Mounting all datasets for ZFS pool '{}'", pool_name);
    
    let result = Exec::cmd("zfs")
        .args(&["mount", "-a"])
        .join();
        
    if result.is_err() || !result.unwrap().success() {
        log::warn!("Failed to mount all datasets for ZFS pool '{}'", pool_name);
        return false;
    }
    true
}

pub fn unmount_all_datasets(pool_name: &str) -> bool {
    log::info!("Unmounting all datasets for ZFS pool '{}'", pool_name);
    
    let result = Exec::cmd("zfs")
        .args(&["unmount", "-a"])
        .join();
        
    if result.is_err() || !result.unwrap().success() {
        log::warn!("Failed to unmount all datasets for ZFS pool '{}'", pool_name);
        return false;
    }
    true
}

pub fn export_pool(pool_name: &str) -> bool {
    log::info!("Exporting ZFS pool '{}'", pool_name);
    
    let result = Exec::cmd("zpool")
        .args(&["export", pool_name])
        .join();
        
    if result.is_err() || !result.unwrap().success() {
        log::warn!("Failed to export ZFS pool '{}'", pool_name);
        return false;
    }
    true
}