#[doc = "Reader of register HSTDMANXTDSC1"]
pub type R = crate::R<u32, super::HSTDMANXTDSC1>;
#[doc = "Writer for register HSTDMANXTDSC1"]
pub type W = crate::W<u32, super::HSTDMANXTDSC1>;
#[doc = "Register HSTDMANXTDSC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::HSTDMANXTDSC1 {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `NXT_DSC_ADD`"]
pub type NXT_DSC_ADD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NXT_DSC_ADD`"]
pub struct NXT_DSC_ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> NXT_DSC_ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&self) -> NXT_DSC_ADD_R {
        NXT_DSC_ADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Next Descriptor Address"]
    #[inline(always)]
    pub fn nxt_dsc_add(&mut self) -> NXT_DSC_ADD_W {
        NXT_DSC_ADD_W { w: self }
    }
}
