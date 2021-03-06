#[doc = "Writer for register SCUPUPD"]
pub type W = crate::W<u32, super::SCUPUPD>;
#[doc = "Write proxy for field `UPRUPD`"]
pub struct UPRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:3 - Update Period Update"]
    #[inline(always)]
    pub fn uprupd(&mut self) -> UPRUPD_W {
        UPRUPD_W { w: self }
    }
}
