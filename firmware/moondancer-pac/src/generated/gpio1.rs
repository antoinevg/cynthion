#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: MODE,
    input: INPUT,
    output: OUTPUT,
    set_clr: SET_CLR,
}
impl RegisterBlock {
    #[doc = "0x00 - Mode register. This :class:`csr.Register` contains an array of ``pin_count`` read/write fields. Each field is 2-bit wide and its possible values are defined by the :class:`PinMode` enumeration. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 16 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 2, \"attr\": \"RW\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins."]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x02 - Input register. This :class:`csr.Register` contains an array of ``pin_count`` read-only fields. Each field is 1-bit wide and driven by the input of its associated pin in the :attr:`Peripheral.pins` array. Values sampled from pin inputs go through :attr:`Peripheral.input_stages` synchronization stages (on a rising edge of ``ClockSignal(\"sync\")``) before reaching the register. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 8 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 1, \"attr\": \"R\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins."]
    #[inline(always)]
    pub const fn input(&self) -> &INPUT {
        &self.input
    }
    #[doc = "0x03 - Output register. This :class:`csr.Register` contains an array of ``pin_count`` read/write fields. Each field is 1-bit wide and drives the output of its associated pin in the :attr:`Peripheral.pins` array, depending on its associated :class:`~Peripheral.Mode` field. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 8 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 1, \"attr\": \"RW\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins."]
    #[inline(always)]
    pub const fn output(&self) -> &OUTPUT {
        &self.output
    }
    #[doc = "0x04 - Output set/clear register. This :class:`csr.Register` contains an array of ``pin_count`` write-only fields. Each field is 2-bit wide; writing it can modify its associated :class:`~Peripheral.Output` field as a side-effect. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 16 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 2, \"attr\": \"W\" }, \\]
- Writing `0b01` to a field sets its associated :class:`~Peripheral.Output` field. - Writing `0b10` to a field clears its associated :class:`~Peripheral.Output` field. - Writing `0b00` or `0b11` to a field has no side-effect. Parameters ---------- pin_count : :class:`int` Number of GPIO pins."]
    #[inline(always)]
    pub const fn set_clr(&self) -> &SET_CLR {
        &self.set_clr
    }
}
#[doc = "Mode (rw) register accessor: Mode register. This :class:`csr.Register` contains an array of ``pin_count`` read/write fields. Each field is 2-bit wide and its possible values are defined by the :class:`PinMode` enumeration. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 16 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 2, \"attr\": \"RW\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "Mode")]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Mode register. This :class:`csr.Register` contains an array of ``pin_count`` read/write fields. Each field is 2-bit wide and its possible values are defined by the :class:`PinMode` enumeration. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 16 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 2, \"attr\": \"RW\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 2, \"attr\": \"RW\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins."]
pub mod mode;
#[doc = "Input (rw) register accessor: Input register. This :class:`csr.Register` contains an array of ``pin_count`` read-only fields. Each field is 1-bit wide and driven by the input of its associated pin in the :attr:`Peripheral.pins` array. Values sampled from pin inputs go through :attr:`Peripheral.input_stages` synchronization stages (on a rising edge of ``ClockSignal(\"sync\")``) before reaching the register. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 8 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 1, \"attr\": \"R\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`input::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input`]
module"]
#[doc(alias = "Input")]
pub type INPUT = crate::Reg<input::INPUT_SPEC>;
#[doc = "Input register. This :class:`csr.Register` contains an array of ``pin_count`` read-only fields. Each field is 1-bit wide and driven by the input of its associated pin in the :attr:`Peripheral.pins` array. Values sampled from pin inputs go through :attr:`Peripheral.input_stages` synchronization stages (on a rising edge of ``ClockSignal(\"sync\")``) before reaching the register. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 8 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 1, \"attr\": \"R\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 1, \"attr\": \"R\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins."]
pub mod input;
#[doc = "Output (rw) register accessor: Output register. This :class:`csr.Register` contains an array of ``pin_count`` read/write fields. Each field is 1-bit wide and drives the output of its associated pin in the :attr:`Peripheral.pins` array, depending on its associated :class:`~Peripheral.Mode` field. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 8 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 1, \"attr\": \"RW\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output`]
module"]
#[doc(alias = "Output")]
pub type OUTPUT = crate::Reg<output::OUTPUT_SPEC>;
#[doc = "Output register. This :class:`csr.Register` contains an array of ``pin_count`` read/write fields. Each field is 1-bit wide and drives the output of its associated pin in the :attr:`Peripheral.pins` array, depending on its associated :class:`~Peripheral.Mode` field. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 8 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 1, \"attr\": \"RW\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 1, \"attr\": \"RW\" }, \\]
Parameters ---------- pin_count : :class:`int` Number of GPIO pins."]
pub mod output;
#[doc = "SetClr (rw) register accessor: Output set/clear register. This :class:`csr.Register` contains an array of ``pin_count`` write-only fields. Each field is 2-bit wide; writing it can modify its associated :class:`~Peripheral.Output` field as a side-effect. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 16 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 2, \"attr\": \"W\" }, \\]
- Writing `0b01` to a field sets its associated :class:`~Peripheral.Output` field. - Writing `0b10` to a field clears its associated :class:`~Peripheral.Output` field. - Writing `0b00` or `0b11` to a field has no side-effect. Parameters ---------- pin_count : :class:`int` Number of GPIO pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`set_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set_clr`]
module"]
#[doc(alias = "SetClr")]
pub type SET_CLR = crate::Reg<set_clr::SET_CLR_SPEC>;
#[doc = "Output set/clear register. This :class:`csr.Register` contains an array of ``pin_count`` write-only fields. Each field is 2-bit wide; writing it can modify its associated :class:`~Peripheral.Output` field as a side-effect. If ``pin_count`` is 8, then the register has the following fields: .. bitfield:: :bits: 16 \\[ { \"name\": \"pin\\[0\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[1\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[2\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[3\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[4\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[5\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[6\\]\", \"bits\": 2, \"attr\": \"W\" }, { \"name\": \"pin\\[7\\]\", \"bits\": 2, \"attr\": \"W\" }, \\]
- Writing `0b01` to a field sets its associated :class:`~Peripheral.Output` field. - Writing `0b10` to a field clears its associated :class:`~Peripheral.Output` field. - Writing `0b00` or `0b11` to a field has no side-effect. Parameters ---------- pin_count : :class:`int` Number of GPIO pins."]
pub mod set_clr;
