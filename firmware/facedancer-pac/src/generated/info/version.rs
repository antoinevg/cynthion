#[doc = "Register `version` reader"]
pub type R = crate::R<VERSION_SPEC>;
#[doc = "Register `version` writer"]
pub type W = crate::W<VERSION_SPEC>;
#[doc = "Field `major` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type MAJOR_R = crate::FieldReader;
#[doc = "Field `minor` reader - TODO amaranth_soc/csr/reg.py:471"]
pub type MINOR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TODO amaranth_soc/csr/reg.py:471"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {}
#[doc = "TODO amaranth_soc/csr/reg.py:471\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERSION_SPEC;
impl crate::RegisterSpec for VERSION_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VERSION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`version::W`](W) writer structure"]
impl crate::Writable for VERSION_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets version to value 0"]
impl crate::Resettable for VERSION_SPEC {
    const RESET_VALUE: u16 = 0;
}