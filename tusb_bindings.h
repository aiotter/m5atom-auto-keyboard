// #if defined(ESP_IDF_COMP_ESPRESSIF__TINYUSB_ENABLED)
// #include "tusb.h"
// #include "class/hid/hid_device.h"
// #endif

#if defined(ESP_IDF_COMP_ESPRESSIF__ESP_TINYUSB_ENABLED)
#include "tinyusb.h"
#include "tinyusb_net.h"
#include "tinyusb_types.h"
#include "tusb_config.h"
#include "tusb_console.h"
#include "tusb_msc_storage.h"
#include "tusb_tasks.h"
#include "vfs_tinyusb.h"

#include "class/hid/hid_device.h"

// Currently you cannot enable CDC
// https://github.com/rust-lang/rust-bindgen/issues/2179
// https://github.com/esp-rs/esp-idf-sys/issues/183
#if CONFIG_TINYUSB_CDC_ENABLED
#include "tusb_cdc_acm.h"
#endif

#endif
