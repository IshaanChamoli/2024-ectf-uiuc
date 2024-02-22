#[doc = "Register `DATAIN16[%s]` reader"]
pub type R = crate::R<DATAIN16_SPEC>;
#[doc = "Register `DATAIN16[%s]` writer"]
pub type W = crate::W<DATAIN16_SPEC>;
#[doc = "Field `DATA` reader - CRC Data"]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - CRC Data"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DATAIN16_SPEC> {
        DATA_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CRC Data Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datain16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datain16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATAIN16_SPEC;
impl crate::RegisterSpec for DATAIN16_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`datain16::R`](R) reader structure"]
impl crate::Readable for DATAIN16_SPEC {}
#[doc = "`write(|w| ..)` method takes [`datain16::W`](W) writer structure"]
impl crate::Writable for DATAIN16_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DATAIN16[%s]
to value 0"]
impl crate::Resettable for DATAIN16_SPEC {
    const RESET_VALUE: u16 = 0;
}
