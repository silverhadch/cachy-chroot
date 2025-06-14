use serde::{Deserialize, Serialize};

pub trait BlockOrSubvolumeID {
    fn get_id(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BlockDevice {
    pub name: String,
    #[serde(rename = "fstype")]
    pub fs_type: String,
    pub uuid: String,
    pub partuuid: Option<String>,
    pub label: Option<String>,
    pub partlabel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zpool_name: Option<String>,
}

impl std::fmt::Display for BlockDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(pool_name) = &self.zpool_name {
            write!(
                f,
                "Partition: {}: FS: {} (ZFS pool: {}) UUID: {}",
                self.name, self.fs_type, pool_name, self.uuid
            )
        } else {
            write!(
                f,
                "Partition: {}: FS: {} UUID: {}",
                self.name, self.fs_type, self.uuid
            )
        }
    }
}

impl BlockOrSubvolumeID for BlockDevice {
    fn get_id(&self) -> String {
        if self.fs_type == "zfs_member" {
            self.zpool_name.clone().unwrap_or_else(|| self.uuid.clone())
        } else {
            self.uuid.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct BTRFSSubVolume {
    pub device: BlockDevice,
    pub subvolume_id: usize,
    pub subvolume_name: String,
}

impl BTRFSSubVolume {
    pub fn new(device: BlockDevice, subvolume_id: usize, subvolume_name: String) -> Self {
        BTRFSSubVolume {
            device,
            subvolume_id,
            subvolume_name,
        }
    }
}

impl std::fmt::Display for BTRFSSubVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{}] BTRFS Subvolume: {}: SubVol ID: {}",
            self.device.name, self.subvolume_name, self.subvolume_id
        )
    }
}

impl BlockOrSubvolumeID for BTRFSSubVolume {
    fn get_id(&self) -> String {
        format!("{}-{}", self.device.get_id(), self.subvolume_id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct BlockDevices {
    #[serde(rename = "blockdevices")]
    pub block_devices: Vec<BlockDevice>,
}