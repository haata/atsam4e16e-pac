#[doc = "Reader of register PMC_USB"]
pub type R = crate::R<u32, super::PMC_USB>;
#[doc = "Writer for register PMC_USB"]
pub type W = crate::W<u32, super::PMC_USB>;
#[doc = "Register PMC_USB `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC_USB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBDIV`"]
pub type USBDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBDIV`"]
pub struct USBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - Divider for USB Clock"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Divider for USB Clock"]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W {
        USBDIV_W { w: self }
    }
}
