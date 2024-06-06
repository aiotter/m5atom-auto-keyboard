use esp_idf_svc::sys::{self, tinyusb};

static mut WL_HANDLE: sys::wl_handle_t = sys::WL_INVALID_HANDLE;

pub fn init() -> anyhow::Result<()> {
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

pub fn unmount() -> Result<(), sys::EspError> {
    sys::esp!(unsafe { tinyusb::tinyusb_msc_storage_unmount() })
}

pub fn deinit() {
    unsafe { tinyusb::tinyusb_msc_storage_deinit() }
}

pub fn is_exposed() -> bool {
    unsafe { tinyusb::tinyusb_msc_storage_in_use_by_usb_host() }
}
