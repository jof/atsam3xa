#[doc = "Reader of register SCUC"]
pub type R = crate::R<u32, super::SCUC>;
#[doc = "Writer for register SCUC"]
pub type W = crate::W<u32, super::SCUC>;
#[doc = "Register SCUC `reset()`'s with value 0"]
impl crate::ResetValue for super::SCUC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
#[doc = "Reader of field `UPDULOCK`"]
pub type UPDULOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPDULOCK`"]
pub struct UPDULOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDULOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Synchronous Channels Update Unlock"]
    #[inline(always)]
    pub fn updulock(&self) -> UPDULOCK_R {
        UPDULOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Channels Update Unlock"]
    #[inline(always)]
    pub fn updulock(&mut self) -> UPDULOCK_W {
        UPDULOCK_W { w: self }
    }
}
