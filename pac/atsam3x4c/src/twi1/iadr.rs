#[doc = "Reader of register IADR"]
pub type R = crate::R<u32, super::IADR>;
#[doc = "Writer for register IADR"]
pub type W = crate::W<u32, super::IADR>;
#[doc = "Register IADR `reset()`'s with value 0"]
impl crate::ResetValue for super::IADR {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `IADR`"]
pub type IADR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IADR`"]
pub struct IADR_W<'a> {
    w: &'a mut W,
}
impl<'a> IADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Internal Address"]
    #[inline(always)]
    pub fn iadr(&self) -> IADR_R {
        IADR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Internal Address"]
    #[inline(always)]
    pub fn iadr(&mut self) -> IADR_W {
        IADR_W { w: self }
    }
}
