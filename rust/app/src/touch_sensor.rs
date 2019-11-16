use embedded_hal::{
    self,
    blocking::delay::DelayMs,
    digital::v2::OutputPin,
};
use mynewt::{
    result::*,
    hw::hal,
    kernel::os,
    sys::console,
};
use crate::mynewt_hal::{
    MynewtDelay,
    MynewtGPIO,
};

/// Probe the I2C bus
pub fn probe() -> MynewtResult<()> {
    for addr in 0..128 {
        let rc = unsafe { hal::hal_i2c_master_probe(1, addr, 1000) };
        if rc != hal::HAL_I2C_ERR_ADDR_NACK as i32 {
            //  I2C device found
            console::print("0x"); console::printhex(addr); console::print(": ");
            console::printhex(rc as u8); console::print("\n"); console::flush();
        }
    }
    console::print("Done\n"); console::flush();
    Ok(())
}

/* I2C devices found:
0x18: 00
0x44: 00 */

pub fn test() -> MynewtResult<()> {
    /*
        let peripherals = nrf52::Peripherals::take().unwrap();
        let nfcpins = peripherals.UICR.nfcpins.read().bits();
        console::print("nfcpins = "); console::printhex(nfcpins as u8); console::print("\n");
    */
    //  Create GPIO for the Reset Pin
    let mut reset = MynewtGPIO::new(10);  //  P0.10/NFC2: TP_RESET. NFC pins must be disabled for this to work.
    let mut delay = MynewtDelay{};

    //  Reset the touch controller by switching the Reset Pin low then high with pauses
    reset.set_low() ? ;
    delay.delay_ms(20);
    reset.set_high() ? ;
    delay.delay_ms(200); delay.delay_ms(200);

    //  Repeat test a few times
    for _ in 0..20 {
        //  I2C addresses to test
        for addr in &[
            0x15,  //  Touch Controller: https://github.com/lupyuen/stm32bluepill-mynewt-sensor/blob/pinetime/hynitron_i2c_cst0xxse_5.1.zip
            //  0x18,  //  Accelerometer: https://ae-bst.resource.bosch.com/media/_tech/media/datasheets/BST-BMA423-DS000.pdf
            //  0x44,  //  Heart Rate Sensor: http://files.pine64.org/doc/datasheet/pinetime/HRS3300%20Heart%20Rate%20Sensor.pdf
        ] {
            //  Registers to be read. Keep tapping the screen to get responses.
            for register in &[
                0x00,  //  Register 0 should have value 0 at startup
                0x01,  //  Not sure what Register 1 does but it shows non-zero values
                0xA3,  //  HYN_REG_CHIP_ID
                0x9F,  //  HYN_REG_CHIP_ID2                    
                0x8F,  //  HYN_REG_INT_CNT
                0xA6,  //  HYN_REG_FW_VER                      
                0xA8,  //  HYN_REG_VENDOR_ID                       
            ] {
                read_register(*addr, *register) ? ;
            }
        }
        console::print("Done\n"); console::flush();
    }
    Ok(())
}

/*
    Touch Controller: https://github.com/lupyuen/stm32bluepill-mynewt-sensor/blob/pinetime/hynitron_i2c_cst0xxse_5.1.zip
    Done
    addr: 0x15, reg: 0x00 = 0x00
    addr: 0x15, reg: 0x01 = 0x05
    addr: 0x15, reg: 0xa3 = 0x00
    Done
    Done
    Done
    Done
    addr: 0x15, reg: 0x00 = 0x00
    addr: 0x15, reg: 0x01 = 0x00
    addr: 0x15, reg: 0xa3 = 0x00
    addr: 0x15, reg: 0x9f = 0x00
    addr: 0x15, reg: 0x8f = 0x00
    addr: 0x15, reg: 0xa6 = 0x00
    addr: 0x15, reg: 0xa8 = 0x00
    Done
    addr: 0x15, reg: 0x00 = 0x00
    addr: 0x15, reg: 0x01 = 0x05
    addr: 0x15, reg: 0xa3 = 0x00
    addr: 0x15, reg: 0x9f = 0x00
    addr: 0x15, reg: 0x8f = 0x00
    addr: 0x15, reg: 0xa6 = 0x00
    addr: 0x15, reg: 0xa8 = 0x00
    Done
    addr: 0x15, reg: 0x00 = 0x00
    addr: 0x15, reg: 0x01 = 0x05
    addr: 0x15, reg: 0xa3 = 0x00
    addr: 0x15, reg: 0x9f = 0x00
    addr: 0x15, reg: 0x8f = 0x00
    addr: 0x15, reg: 0xa6 = 0x00
    addr: 0x15, reg: 0xa8 = 0x00
    Done
    addr: 0x15, reg: 0x00 = 0x00
    addr: 0x15, reg: 0x01 = 0x00
    addr: 0x15, reg: 0xa3 = 0x00
    addr: 0x15, reg: 0x9f = 0x00
    addr: 0x15, reg: 0x8f = 0x00
    addr: 0x15, reg: 0xa6 = 0x00
    addr: 0x15, reg: 0xa8 = 0x00
    Done
    addr: 0x15, reg: 0x00 = 0x00
    addr: 0x15, reg: 0x01 = 0x00
    addr: 0x15, reg: 0xa3 = 0x00
    addr: 0x15, reg: 0x9f = 0x00
    addr: 0x15, reg: 0x8f = 0x00
    addr: 0x15, reg: 0xa6 = 0x00
    addr: 0x15, reg: 0xa8 = 0x00
    Done
    addr: 0x15, reg: 0x00 = 0x00
    Done
    Done
    Done
    addr: 0x15, reg: 0x00 = 0x00
    addr: 0x15, reg: 0x01 = 0x04
    Done
    Done
    Done
    Done
    addr: 0x15, reg: 0x00 = 0x00
    Done
    Done
    Done

    Accelerometer: https://ae-bst.resource.bosch.com/media/_tech/media/datasheets/BST-BMA423-DS000.pdf
    addr: 0x18, reg: 0x00 = 0x11
    addr: 0x18, reg: 0xa3 = 0x00
    addr: 0x18, reg: 0x9f = 0x00
    addr: 0x18, reg: 0x8f = 0x00
    addr: 0x18, reg: 0xa6 = 0x80
    addr: 0x18, reg: 0xa8 = 0x00

    Heart Rate Sensor: http://files.pine64.org/doc/datasheet/pinetime/HRS3300%20Heart%20Rate%20Sensor.pdf
    addr: 0x44, reg: 0x00 = 0x21
    addr: 0x44, reg: 0xa3 = 0xa3
    addr: 0x44, reg: 0x9f = 0xa3
    addr: 0x44, reg: 0x8f = 0xa3
    addr: 0x44, reg: 0xa6 = 0xa3
    addr: 0x44, reg: 0xa8 = 0xa3
*/

/// Read the I2C register for the specified I2C address (7-bit address)
fn read_register(addr: u8, register: u8) -> MynewtResult<()> {
    //  First the register address must be sent in write mode (slave address xxxxxxx0). 
    unsafe { 
        I2C_BUFFER[0] = register;
        I2C_DATA.address = addr;
        I2C_DATA.len = I2C_BUFFER.len() as u16;
        I2C_DATA.buffer = I2C_BUFFER.as_mut_ptr();
    };
    //  Then either a stop or a repeated start condition must be generated. 
    let rc1 = unsafe { hal::hal_i2c_master_write(1, &mut I2C_DATA, 1000, 0) };  //  No stop
    //  let rc1 = unsafe { hal::hal_i2c_master_write(1, &mut I2C_DATA, 1000, 1) };  //  With stop
    /* if rc1 == hal::HAL_I2C_ERR_ADDR_NACK as i32 {
        return Ok(());
    } */
    //  After this the slave is addressed in read mode (RW = ‘1’) at address xxxxxxx1, 
    unsafe { 
        I2C_BUFFER[0] = 0x00;
        I2C_DATA.address = addr;
        I2C_DATA.len = I2C_BUFFER.len() as u16;
        I2C_DATA.buffer = I2C_BUFFER.as_mut_ptr();
    };
    //  After which the slave sends out data from auto-incremented register addresses until a NOACKM and stop condition occurs.
    let rc2 = unsafe { hal::hal_i2c_master_read(1, &mut I2C_DATA, 1000, 1) };
    if rc2 == hal::HAL_I2C_ERR_ADDR_NACK as i32 {
        return Ok(());
    }
    console::print("addr: 0x"); console::printhex(addr); 
    console::print(", reg: 0x"); console::printhex(register); 
    console::print(" = 0x"); console::printhex(unsafe { I2C_BUFFER[0] }); 
    console::print("\n"); console::flush();
    Ok(())
}

static mut I2C_BUFFER: [u8; 1] =  [ 0 ];

static mut I2C_DATA: hal::hal_i2c_master_data = hal::hal_i2c_master_data {
    address: 0,
    len:     0,
    buffer:  core::ptr::null_mut(),
};

/*
write 0x18: 00
read 0x18: 00
value 0x00
write 0x44: 00
read 0x44: 00
value 0xa3
write 0x98: 00
read 0x98: 00
value 0x00
write 0xc4: 00
read 0xc4: 00
value 0xa3
*/

/*
From http://www.hynitron.com/upload/1408075960.pdf:
//初始化设置
I2Cm_Write_Data(0xAF, 0x3A); //打开写保护
I2Cm_Write_Data(0x1F,0x00); //关闭扫描
I2Cm_Write_Data(0x12, 0x18); //设置允许多个button被同时触发，调试时关闭噪声保护

//IRQ处理
// 读取所有Button的状态
Button=I2Cm_Read_Data(0x22); //读取感应通道0-7的状态寄存器用于判断其触摸状态
I2Cm_Write_Data(0x1F, 0x01); //清除IRQ（不打开写保护时仅能清除IRQ位，其它控制位不变）
// 客户可以根据Button的信息读取相关感应通道的Signal值并对数据进行进一步处理
*/