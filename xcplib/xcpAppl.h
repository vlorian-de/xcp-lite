#pragma once
/* xcpAppl.h */

extern void ApplXcpSetLogLevel(uint8_t level);

extern void ApplXcpRegisterCallbacks(
    uint8_t (*cb_connect)(),
    uint8_t (*cb_get_cal_page)(uint8_t segment, uint8_t mode),
    uint8_t (*cb_set_cal_page)(uint8_t segment, uint8_t page, uint8_t mode),
    uint8_t (*cb_freeze_cal)(),
    uint8_t (*cb_init_cal)(uint8_t src_page,uint8_t dst_page),
    uint8_t (*cb_read)(uint32_t src, uint8_t size, uint8_t* dst),
    uint8_t (*cb_write)(uint32_t dst, uint8_t size, const uint8_t* src, uint8_t delay),
    uint8_t (*cb_flush)()
);

extern void ApplXcpSetA2lName(const char *name);