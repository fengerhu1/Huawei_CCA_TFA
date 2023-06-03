use crate::util::list::ListHead;

/// Represent the type of a ipa_region
#[repr(C)]
pub enum IrType {
    IrEagerMapping = 0,
    IrLazyMapping,
}

/// IPA region, records the GPA regions of stage-2 mmu
/// corresponding c version struct is ipa_region
#[repr(C)]
pub struct IpaRegion {
    pub region_node: ListHead,
    pub ipa_start: u64,
    pub pa_start: u64,
    pub size: u64,
    pub region_attr: u32,
    pub region_type: u32,
}

impl IpaRegion {
    pub fn new(
        ipa_start: u64,
        pa_start: u64,
        size: u64,
        region_attr: u32,
        region_type: u32,
    ) -> Self {
        let list = ListHead::new();
        IpaRegion {
            region_node: list,
            ipa_start,
            pa_start,
            size,
            region_attr,
            region_type,
        }
    }

    pub fn contains_ipa(&self, ipa: u64) -> bool {
        (self.ipa_start <= ipa) && (ipa < self.ipa_start + self.size)
    }
}
