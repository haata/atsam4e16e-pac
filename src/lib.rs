#![doc = "Peripheral access API for ATSAM4E16E microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn ID_PMC();
    fn ID_EFC();
    fn ID_UART0();
    fn ID_PIOA();
    fn ID_PIOB();
    fn ID_PIOC();
    fn ID_PIOD();
    fn ID_PIOE();
    fn ID_USART0();
    fn ID_USART1();
    fn ID_HSMCI();
    fn ID_TWI0();
    fn ID_TWI1();
    fn ID_SPI();
    fn ID_DMAC();
    fn ID_TC0();
    fn ID_TC1();
    fn ID_TC2();
    fn ID_TC3();
    fn ID_TC4();
    fn ID_TC5();
    fn ID_TC6();
    fn ID_TC7();
    fn ID_TC8();
    fn ID_AFEC0();
    fn ID_AFEC1();
    fn ID_DACC();
    fn ID_ACC();
    fn ID_UDP();
    fn ID_PWM();
    fn ID_CAN0();
    fn ID_CAN1();
    fn ID_AES();
    fn ID_GMAC();
    fn ID_UART1();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 46] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ID_PMC },
    Vector { _handler: ID_EFC },
    Vector { _handler: ID_UART0 },
    Vector { _reserved: 0 },
    Vector { _handler: ID_PIOA },
    Vector { _handler: ID_PIOB },
    Vector { _handler: ID_PIOC },
    Vector { _handler: ID_PIOD },
    Vector { _handler: ID_PIOE },
    Vector {
        _handler: ID_USART0,
    },
    Vector {
        _handler: ID_USART1,
    },
    Vector { _handler: ID_HSMCI },
    Vector { _handler: ID_TWI0 },
    Vector { _handler: ID_TWI1 },
    Vector { _handler: ID_SPI },
    Vector { _handler: ID_DMAC },
    Vector { _handler: ID_TC0 },
    Vector { _handler: ID_TC1 },
    Vector { _handler: ID_TC2 },
    Vector { _handler: ID_TC3 },
    Vector { _handler: ID_TC4 },
    Vector { _handler: ID_TC5 },
    Vector { _handler: ID_TC6 },
    Vector { _handler: ID_TC7 },
    Vector { _handler: ID_TC8 },
    Vector { _handler: ID_AFEC0 },
    Vector { _handler: ID_AFEC1 },
    Vector { _handler: ID_DACC },
    Vector { _handler: ID_ACC },
    Vector { _reserved: 0 },
    Vector { _handler: ID_UDP },
    Vector { _handler: ID_PWM },
    Vector { _handler: ID_CAN0 },
    Vector { _handler: ID_CAN1 },
    Vector { _handler: ID_AES },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ID_GMAC },
    Vector { _handler: ID_UART1 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "5 - ID_PMC"]
    ID_PMC = 5,
    #[doc = "6 - ID_EFC"]
    ID_EFC = 6,
    #[doc = "7 - ID_UART0"]
    ID_UART0 = 7,
    #[doc = "9 - ID_PIOA"]
    ID_PIOA = 9,
    #[doc = "10 - ID_PIOB"]
    ID_PIOB = 10,
    #[doc = "11 - ID_PIOC"]
    ID_PIOC = 11,
    #[doc = "12 - ID_PIOD"]
    ID_PIOD = 12,
    #[doc = "13 - ID_PIOE"]
    ID_PIOE = 13,
    #[doc = "14 - ID_USART0"]
    ID_USART0 = 14,
    #[doc = "15 - ID_USART1"]
    ID_USART1 = 15,
    #[doc = "16 - ID_HSMCI"]
    ID_HSMCI = 16,
    #[doc = "17 - ID_TWI0"]
    ID_TWI0 = 17,
    #[doc = "18 - ID_TWI1"]
    ID_TWI1 = 18,
    #[doc = "19 - ID_SPI"]
    ID_SPI = 19,
    #[doc = "20 - ID_DMAC"]
    ID_DMAC = 20,
    #[doc = "21 - ID_TC0"]
    ID_TC0 = 21,
    #[doc = "22 - ID_TC1"]
    ID_TC1 = 22,
    #[doc = "23 - ID_TC2"]
    ID_TC2 = 23,
    #[doc = "24 - ID_TC3"]
    ID_TC3 = 24,
    #[doc = "25 - ID_TC4"]
    ID_TC4 = 25,
    #[doc = "26 - ID_TC5"]
    ID_TC5 = 26,
    #[doc = "27 - ID_TC6"]
    ID_TC6 = 27,
    #[doc = "28 - ID_TC7"]
    ID_TC7 = 28,
    #[doc = "29 - ID_TC8"]
    ID_TC8 = 29,
    #[doc = "30 - ID_AFEC0"]
    ID_AFEC0 = 30,
    #[doc = "31 - ID_AFEC1"]
    ID_AFEC1 = 31,
    #[doc = "32 - ID_DACC"]
    ID_DACC = 32,
    #[doc = "33 - ID_ACC"]
    ID_ACC = 33,
    #[doc = "35 - ID_UDP"]
    ID_UDP = 35,
    #[doc = "36 - ID_PWM"]
    ID_PWM = 36,
    #[doc = "37 - ID_CAN0"]
    ID_CAN0 = 37,
    #[doc = "38 - ID_CAN1"]
    ID_CAN1 = 38,
    #[doc = "39 - ID_AES"]
    ID_AES = 39,
    #[doc = "44 - ID_GMAC"]
    ID_GMAC = 44,
    #[doc = "45 - ID_UART1"]
    ID_UART1 = 45,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Pulse Width Modulation Controller"]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM::ptr() }
    }
}
#[doc = "Pulse Width Modulation Controller"]
pub mod pwm;
#[doc = "Advanced Encryption Standard"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "Advanced Encryption Standard"]
pub mod aes;
#[doc = "Controller Area Network 0"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "Controller Area Network 0"]
pub mod can0;
#[doc = "Controller Area Network 1"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Controller Area Network 1"]
pub mod can1;
#[doc = "Gigabit Ethernet MAC"]
pub struct GMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GMAC {}
impl GMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gmac::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for GMAC {
    type Target = gmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GMAC::ptr() }
    }
}
#[doc = "Gigabit Ethernet MAC"]
pub mod gmac;
#[doc = "Cyclic Redundancy Check Calculation Unit"]
pub struct CRCCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRCCU {}
impl CRCCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crccu::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for CRCCU {
    type Target = crccu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRCCU::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check Calculation Unit"]
pub mod crccu;
#[doc = "Static Memory Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smc::RegisterBlock {
        0x4006_0000 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "Static Memory Controller"]
pub mod smc;
#[doc = "Universal Asynchronous Receiver Transmitter 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x4006_0600 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 1"]
pub mod uart1;
#[doc = "High Speed MultiMedia Card Interface"]
pub struct HSMCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSMCI {}
impl HSMCI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsmci::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for HSMCI {
    type Target = hsmci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HSMCI::ptr() }
    }
}
#[doc = "High Speed MultiMedia Card Interface"]
pub mod hsmci;
#[doc = "USB Device Port"]
pub struct UDP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDP {}
impl UDP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const udp::RegisterBlock {
        0x4008_4000 as *const _
    }
}
impl Deref for UDP {
    type Target = udp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UDP::ptr() }
    }
}
#[doc = "USB Device Port"]
pub mod udp;
#[doc = "Serial Peripheral Interface"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi;
#[doc = "Timer Counter 0"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4009_0000 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Timer Counter 0"]
pub mod tc0;
#[doc = "Timer Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc1::RegisterBlock {
        0x4009_4000 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "Timer Counter 1"]
pub mod tc1;
#[doc = "Timer Counter 2"]
pub struct TC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC2 {}
impl TC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc2::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for TC2 {
    type Target = tc2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC2::ptr() }
    }
}
#[doc = "Timer Counter 2"]
pub mod tc2;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x400a_0000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub mod usart0;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x400a_4000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub mod usart1;
#[doc = "Two-wire Interface 0"]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi0::RegisterBlock {
        0x400a_8000 as *const _
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWI0::ptr() }
    }
}
#[doc = "Two-wire Interface 0"]
pub mod twi0;
#[doc = "Two-wire Interface 1"]
pub struct TWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI1 {}
impl TWI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi1::RegisterBlock {
        0x400a_c000 as *const _
    }
}
impl Deref for TWI1 {
    type Target = twi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWI1::ptr() }
    }
}
#[doc = "Two-wire Interface 1"]
pub mod twi1;
#[doc = "Analog-Front-End Controller 0"]
pub struct AFEC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFEC0 {}
impl AFEC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afec0::RegisterBlock {
        0x400b_0000 as *const _
    }
}
impl Deref for AFEC0 {
    type Target = afec0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AFEC0::ptr() }
    }
}
#[doc = "Analog-Front-End Controller 0"]
pub mod afec0;
#[doc = "Analog-Front-End Controller 1"]
pub struct AFEC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFEC1 {}
impl AFEC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afec1::RegisterBlock {
        0x400b_4000 as *const _
    }
}
impl Deref for AFEC1 {
    type Target = afec1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AFEC1::ptr() }
    }
}
#[doc = "Analog-Front-End Controller 1"]
pub mod afec1;
#[doc = "Digital-to-Analog Converter Controller"]
pub struct DACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DACC {}
impl DACC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dacc::RegisterBlock {
        0x400b_8000 as *const _
    }
}
impl Deref for DACC {
    type Target = dacc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DACC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter Controller"]
pub mod dacc;
#[doc = "Analog Comparator Controller"]
pub struct ACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACC {}
impl ACC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acc::RegisterBlock {
        0x400b_c000 as *const _
    }
}
impl Deref for ACC {
    type Target = acc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACC::ptr() }
    }
}
#[doc = "Analog Comparator Controller"]
pub mod acc;
#[doc = "DMA Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac::RegisterBlock {
        0x400c_0000 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "DMA Controller"]
pub mod dmac;
#[doc = "Cortex M Cache Controller"]
pub struct CMCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMCC {}
impl CMCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmcc::RegisterBlock {
        0x400c_4000 as *const _
    }
}
impl Deref for CMCC {
    type Target = cmcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMCC::ptr() }
    }
}
#[doc = "Cortex M Cache Controller"]
pub mod cmcc;
#[doc = "AHB Bus Matrix"]
pub struct MATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MATRIX {}
impl MATRIX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const matrix::RegisterBlock {
        0x400e_0200 as *const _
    }
}
impl Deref for MATRIX {
    type Target = matrix::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MATRIX::ptr() }
    }
}
#[doc = "AHB Bus Matrix"]
pub mod matrix;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        0x400e_0400 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "Universal Asynchronous Receiver Transmitter 0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x400e_0600 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 0"]
pub mod uart0;
#[doc = "Chip Identifier"]
pub struct CHIPID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CHIPID {}
impl CHIPID {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const chipid::RegisterBlock {
        0x400e_0740 as *const _
    }
}
impl Deref for CHIPID {
    type Target = chipid::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CHIPID::ptr() }
    }
}
#[doc = "Chip Identifier"]
pub mod chipid;
#[doc = "Embedded Flash Controller"]
pub struct EFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFC {}
impl EFC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efc::RegisterBlock {
        0x400e_0a00 as *const _
    }
}
impl Deref for EFC {
    type Target = efc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFC::ptr() }
    }
}
#[doc = "Embedded Flash Controller"]
pub mod efc;
#[doc = "Parallel Input/Output Controller A"]
pub struct PIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOA {}
impl PIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioa::RegisterBlock {
        0x400e_0e00 as *const _
    }
}
impl Deref for PIOA {
    type Target = pioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOA::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller A"]
pub mod pioa;
#[doc = "Parallel Input/Output Controller B"]
pub struct PIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOB {}
impl PIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const piob::RegisterBlock {
        0x400e_1000 as *const _
    }
}
impl Deref for PIOB {
    type Target = piob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOB::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller B"]
pub mod piob;
#[doc = "Parallel Input/Output Controller C"]
pub struct PIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOC {}
impl PIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioc::RegisterBlock {
        0x400e_1200 as *const _
    }
}
impl Deref for PIOC {
    type Target = pioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOC::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller C"]
pub mod pioc;
#[doc = "Parallel Input/Output Controller D"]
pub struct PIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOD {}
impl PIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const piod::RegisterBlock {
        0x400e_1400 as *const _
    }
}
impl Deref for PIOD {
    type Target = piod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOD::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller D"]
pub mod piod;
#[doc = "Parallel Input/Output Controller E"]
pub struct PIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOE {}
impl PIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioe::RegisterBlock {
        0x400e_1600 as *const _
    }
}
impl Deref for PIOE {
    type Target = pioe::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOE::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller E"]
pub mod pioe;
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstc::RegisterBlock {
        0x400e_1800 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const supc::RegisterBlock {
        0x400e_1810 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Real-time Timer"]
pub struct RTT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTT {}
impl RTT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtt::RegisterBlock {
        0x400e_1830 as *const _
    }
}
impl Deref for RTT {
    type Target = rtt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTT::ptr() }
    }
}
#[doc = "Real-time Timer"]
pub mod rtt;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x400e_1850 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Real-time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x400e_1860 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time Clock"]
pub mod rtc;
#[doc = "General Purpose Backup Registers"]
pub struct GPBR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPBR {}
impl GPBR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpbr::RegisterBlock {
        0x400e_1890 as *const _
    }
}
impl Deref for GPBR {
    type Target = gpbr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPBR::ptr() }
    }
}
#[doc = "General Purpose Backup Registers"]
pub mod gpbr;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "GMAC"]
    pub GMAC: GMAC,
    #[doc = "CRCCU"]
    pub CRCCU: CRCCU,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "HSMCI"]
    pub HSMCI: HSMCI,
    #[doc = "UDP"]
    pub UDP: UDP,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TC2"]
    pub TC2: TC2,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "AFEC0"]
    pub AFEC0: AFEC0,
    #[doc = "AFEC1"]
    pub AFEC1: AFEC1,
    #[doc = "DACC"]
    pub DACC: DACC,
    #[doc = "ACC"]
    pub ACC: ACC,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "CMCC"]
    pub CMCC: CMCC,
    #[doc = "MATRIX"]
    pub MATRIX: MATRIX,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "CHIPID"]
    pub CHIPID: CHIPID,
    #[doc = "EFC"]
    pub EFC: EFC,
    #[doc = "PIOA"]
    pub PIOA: PIOA,
    #[doc = "PIOB"]
    pub PIOB: PIOB,
    #[doc = "PIOC"]
    pub PIOC: PIOC,
    #[doc = "PIOD"]
    pub PIOD: PIOD,
    #[doc = "PIOE"]
    pub PIOE: PIOE,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "RTT"]
    pub RTT: RTT,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "GPBR"]
    pub GPBR: GPBR,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PWM: PWM {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            GMAC: GMAC {
                _marker: PhantomData,
            },
            CRCCU: CRCCU {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            HSMCI: HSMCI {
                _marker: PhantomData,
            },
            UDP: UDP {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TC2: TC2 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            TWI1: TWI1 {
                _marker: PhantomData,
            },
            AFEC0: AFEC0 {
                _marker: PhantomData,
            },
            AFEC1: AFEC1 {
                _marker: PhantomData,
            },
            DACC: DACC {
                _marker: PhantomData,
            },
            ACC: ACC {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            CMCC: CMCC {
                _marker: PhantomData,
            },
            MATRIX: MATRIX {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            CHIPID: CHIPID {
                _marker: PhantomData,
            },
            EFC: EFC {
                _marker: PhantomData,
            },
            PIOA: PIOA {
                _marker: PhantomData,
            },
            PIOB: PIOB {
                _marker: PhantomData,
            },
            PIOC: PIOC {
                _marker: PhantomData,
            },
            PIOD: PIOD {
                _marker: PhantomData,
            },
            PIOE: PIOE {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            RTT: RTT {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            GPBR: GPBR {
                _marker: PhantomData,
            },
        }
    }
}
