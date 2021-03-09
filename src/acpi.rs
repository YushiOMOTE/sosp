use crate::memory::phys_to_virt_as_ptr;
use acpi::{platform::interrupt::InterruptModel, AcpiHandler, AcpiTables, PhysicalMapping};
use core::ptr::NonNull;
use log::*;
use x86_64::{PhysAddr, VirtAddr};

#[derive(Clone)]
struct Handler {
    physical_memory_offset: VirtAddr,
}

impl Handler {
    fn new(physical_memory_offset: VirtAddr) -> Self {
        Self {
            physical_memory_offset,
        }
    }
}

impl AcpiHandler for Handler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        let virtual_start = phys_to_virt_as_ptr(
            self.physical_memory_offset,
            PhysAddr::new(physical_address as u64),
        );

        PhysicalMapping {
            physical_start: physical_address,
            virtual_start: NonNull::new(virtual_start).unwrap(),
            region_length: size,
            mapped_length: size,
            handler: self.clone(),
        }
    }

    fn unmap_physical_region<T>(&self, _region: &PhysicalMapping<Self, T>) {}
}

pub fn init_acpi(physical_memory_offset: VirtAddr) {
    let handler = Handler::new(physical_memory_offset);
    let acpi = unsafe { AcpiTables::search_for_rsdp_bios(handler).unwrap() };

    info!(
        "Interrupt model: {:#?}",
        acpi.platform_info().unwrap().interrupt_model
    );

    if let InterruptModel::Apic(apic) = acpi.platform_info().unwrap().interrupt_model {
        let addr = apic.local_apic_address;
        let mut apicbase = unsafe {
            apic::ApicBase::new(phys_to_virt_as_ptr(
                physical_memory_offset,
                PhysAddr::new(addr),
            ))
        };
        info!("APIC ID: {:?}", apicbase.id().read());
        info!("APIC Version: {:?}", apicbase.version().read());
    } else {
        error!("APIC not found");
    }
}
