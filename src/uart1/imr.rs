#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDRX`"]
pub type ENDRX_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDTX`"]
pub type ENDTX_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE`"]
pub type OVRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FRAME`"]
pub type FRAME_R = crate::R<bool, bool>;
#[doc = "Reader of field `PARE`"]
pub type PARE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBUFE`"]
pub type TXBUFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBUFF`"]
pub type RXBUFF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Receive Buffer Interrupt Mask (available in all USART modes of operation)"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Transmit Buffer Interrupt Mask (available in all USART modes of operation)"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mask TXBUFE Interrupt"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
