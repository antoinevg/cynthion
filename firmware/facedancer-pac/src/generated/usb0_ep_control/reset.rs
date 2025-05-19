#[doc = "Register `reset` writer"]
pub type W = crate::W<RESET_SPEC>;
#[doc = "Field `reset` writer - Local reset control for the SETUP handler; writing a '1' to this register clears the handler state."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Local reset control for the SETUP handler; writing a '1' to this register clears the handler state."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<RESET_SPEC> {
        RESET_W::new(self, 0)
    }
}
#[doc = "usb0_ep_control reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_SPEC;
impl crate::RegisterSpec for RESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reset::W`](W) writer structure"]
impl crate::Writable for RESET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets reset to value 0"]
impl crate::Resettable for RESET_SPEC {
    const RESET_VALUE: u32 = 0;
}
