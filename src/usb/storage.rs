use esp_idf_svc::sys::{self, tinyusb};

static mut WL_HANDLE: sys::wl_handle_t = sys::WL_INVALID_HANDLE;

pub fn ensure_wl() -> Result<(), sys::EspError> {
    if unsafe { WL_HANDLE != sys::WL_INVALID_HANDLE } {
        return Ok(());
    };

    let data_partition = unsafe {
        sys::esp_partition_find_first(
            tinyusb::esp_partition_type_t_ESP_PARTITION_TYPE_DATA,
            tinyusb::esp_partition_subtype_t_ESP_PARTITION_SUBTYPE_DATA_FAT,
            std::ptr::null(),
        )
    };

    if data_partition == std::ptr::null() {
        log::error!("Failed to find FATFS partition. Check the partition table.");
        sys::esp!(sys::ESP_ERR_NOT_FOUND as sys::esp_err_t)?;
    }

    sys::esp!(unsafe { sys::wl_mount(data_partition, std::ptr::addr_of_mut!(WL_HANDLE)) })?;
    Ok(())
}

pub fn init_msc() -> anyhow::Result<()> {
    ensure_wl()?;

    let mut config_spi: tinyusb::tinyusb_msc_spiflash_config_t = unsafe { std::mem::zeroed() };
    config_spi.wl_handle = unsafe { WL_HANDLE };
    config_spi.mount_config.format_if_mount_failed = true;

    sys::esp!(unsafe {
        tinyusb::tinyusb_msc_storage_init_spiflash(std::ptr::from_ref(&config_spi))
    })?;

    log::info!("MSC initialized");
    Ok(())
}

pub fn mount(mount_path: &std::ffi::CStr) -> Result<(), sys::EspError> {
    sys::esp!(unsafe { tinyusb::tinyusb_msc_storage_mount(mount_path.as_ptr()) })
}

pub fn mount_without_msc(
    mount_path: &str,
) -> Result<esp_idf_svc::io::vfs::MountedFatfs<esp_idf_svc::fs::fatfs::Fatfs<()>>, sys::EspError> {
    ensure_wl()?;

    let drive = {
        let mut pdrv: sys::BYTE = 0xff;
        sys::esp!(unsafe { sys::ff_diskio_get_drive(&mut pdrv) })?;
        pdrv
    };

    let fs = unsafe { esp_idf_svc::fs::fatfs::Fatfs::new_wl_part(drive, WL_HANDLE)? };
    esp_idf_svc::io::vfs::MountedFatfs::mount(fs, mount_path, 1)
}

pub struct MountedFs {
    drive: u8,
    path: std::ffi::CString,
}

impl MountedFs {
    pub fn mount(mount_path: &std::ffi::CStr) -> Result<Self, sys::EspError> {
        ensure_wl()?;

        let drive = {
            let mut pdrv: sys::BYTE = 0xff;
            sys::esp!(unsafe { sys::ff_diskio_get_drive(&mut pdrv) })?;
            pdrv
        };
        let fs: Box<sys::FATFS> = Box::new(unsafe { std::mem::zeroed() });
        let drive_name: [core::ffi::c_char; 2] = [(drive + '0' as u8) as _, 0];
        let max_files = 10;
        sys::esp!(unsafe {
            sys::esp_vfs_fat_register(
                mount_path.as_ptr(),
                drive_name.as_ptr(),
                max_files,
                &mut Box::into_raw(fs),
            )
        })?;

        Ok(Self {
            path: mount_path.into(),
            drive,
        })
    }
}

impl Drop for MountedFs {
    fn drop(&mut self) {
        unsafe {
            sys::f_mount(core::ptr::null_mut(), self.path.as_ptr(), 0);
        }

        sys::esp!(unsafe { sys::esp_vfs_fat_unregister_path(self.path.as_ptr()) }).unwrap();
    }
}

pub fn unmount() -> Result<(), sys::EspError> {
    sys::esp!(unsafe { tinyusb::tinyusb_msc_storage_unmount() })
}

pub fn deinit() {
    unsafe { tinyusb::tinyusb_msc_storage_deinit() }
}

pub fn is_exposed() -> bool {
    unsafe { tinyusb::tinyusb_msc_storage_in_use_by_usb_host() }
}
