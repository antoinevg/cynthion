#[doc = "Register `ev_pending` reader"]
pub type R = crate::R<EV_PENDING_SPEC>;
#[doc = "Register `ev_pending` writer"]
pub type W = crate::W<EV_PENDING_SPEC>;
#[doc = "Field `pending` reader - uart1 pending register field"]
pub type PENDING_R = crate::FieldReader;
#[doc = "Field `pending` writer - uart1 pending register field"]
pub type PENDING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - uart1 pending register field"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - uart1 pending register field"]
    #[inline(always)]
    #[must_use]
    pub fn pending(&mut self) -> PENDING_W<EV_PENDING_SPEC> {
        PENDING_W::new(self, 0)
    }
}
#[doc = "uart1 ev_pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ev_pending::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EV_PENDING_SPEC;
impl crate::RegisterSpec for EV_PENDING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_pending::R`](R) reader structure"]
impl crate::Readable for EV_PENDING_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ev_pending::W`](W) writer structure"]
impl crate::Writable for EV_PENDING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ev_pending to value 0"]
impl crate::Resettable for EV_PENDING_SPEC {
    const RESET_VALUE: u32 = 0;
}
