//! USB Descriptors

#![allow(non_snake_case)]

use core::iter;
use core::marker::PhantomData;
use core::mem::size_of;
use core::slice;

use zerocopy::{AsBytes, FromBytes, FromZeroes};

use crate::traits::AsByteSliceIterator;

/// USB descriptor type.
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum DescriptorType {
    Device = 1,
    Configuration = 2,
    String = 3,
    Interface = 4,
    Endpoint = 5,
    DeviceQualifier = 6,
    OtherSpeedConfiguration = 7,
    InterfacePower = 8,
    OnTheGo = 9,
    Debug = 10,
    InterfaceAssociation = 11,
    Security = 12,
    Key = 13,
    EncryptionType = 14,
    BinaryDeviceObjectStore = 15,
    DeviceCapability = 16,
    WirelessEndpointCompanion = 17,
    SuperSpeedEndpointCompanion = 48,
    Unknown = 0xff,
}

impl From<u8> for DescriptorType {
    fn from(value: u8) -> Self {
        match value {
            1 => DescriptorType::Device,
            2 => DescriptorType::Configuration,
            3 => DescriptorType::String,
            4 => DescriptorType::Interface,
            5 => DescriptorType::Endpoint,
            6 => DescriptorType::DeviceQualifier,
            7 => DescriptorType::OtherSpeedConfiguration,
            8 => DescriptorType::InterfacePower,
            9 => DescriptorType::OnTheGo,
            10 => DescriptorType::Debug,
            11 => DescriptorType::InterfaceAssociation,
            12 => DescriptorType::Security,
            13 => DescriptorType::Key,
            14 => DescriptorType::EncryptionType,
            15 => DescriptorType::BinaryDeviceObjectStore,
            16 => DescriptorType::DeviceCapability,
            17 => DescriptorType::WirelessEndpointCompanion,
            48 => DescriptorType::SuperSpeedEndpointCompanion,
            _ => DescriptorType::Unknown,
        }
    }
}

// - DeviceDescriptor ---------------------------------------------------------

/// USB device descriptor
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy)]
#[repr(C, packed)]
pub struct DeviceDescriptor {
    pub bLength: u8,         // 18
    pub bDescriptorType: u8, // 1 = Device
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}

impl AsByteSliceIterator for DeviceDescriptor {}

impl DeviceDescriptor {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new() -> Self {
        Self {
            bLength: size_of::<Self>() as u8,
            bDescriptorType: DescriptorType::Device as u8,
            bcdUSB: 0x0200,
            bDeviceClass: 0,
            bDeviceSubClass: 0,
            bDeviceProtocol: 0,
            bMaxPacketSize: 0,
            idVendor: 0,
            idProduct: 0,
            bcdDevice: 0,
            iManufacturer: 0,
            iProduct: 0,
            iSerialNumber: 0,
            bNumConfigurations: 0,
        }
    }
}

impl Default for DeviceDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// - DeviceQualifierDescriptor ------------------------------------------------

/// USB device qualifier descriptor
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy)]
#[repr(C, packed)]
pub struct DeviceQualifierDescriptor {
    pub bLength: u8,         // 10
    pub bDescriptorType: u8, // 6 = DeviceQualifier
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize0: u8,
    pub bNumConfigurations: u8,
    pub bReserved: u8,
}

impl AsByteSliceIterator for DeviceQualifierDescriptor {}

impl DeviceQualifierDescriptor {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new() -> Self {
        Self {
            bLength: size_of::<Self>() as u8,
            bDescriptorType: DescriptorType::DeviceQualifier as u8,
            bcdUSB: 0,
            bDeviceClass: 0,
            bDeviceSubClass: 0,
            bDeviceProtocol: 0,
            bMaxPacketSize0: 0,
            bNumConfigurations: 0,
            bReserved: 0,
        }
    }
}

impl Default for DeviceQualifierDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// - ConfigurationDescriptor --------------------------------------------------

/// USB configuration descriptor header
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy)]
#[repr(C, packed)]
pub struct ConfigurationDescriptorHeader {
    pub bLength: u8,         // 9
    pub bDescriptorType: u8, // 2 = Configuration, 3 = OtherSpeedConfiguration TODO
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub bMaxPower: u8,
}

impl AsByteSliceIterator for ConfigurationDescriptorHeader {}

impl ConfigurationDescriptorHeader {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new() -> Self {
        Self {
            bLength: size_of::<Self>() as u8,
            bDescriptorType: DescriptorType::Configuration as u8,
            wTotalLength: 0,
            bNumInterfaces: 0,
            bConfigurationValue: 0,
            iConfiguration: 0,
            bmAttributes: 0,
            bMaxPower: 0,
        }
    }
}

/// USB configuration descriptor
#[derive(Clone, Copy)]
pub struct ConfigurationDescriptor<'a> {
    pub head: ConfigurationDescriptorHeader,
    pub tail: &'a [InterfaceDescriptor<'a>],
}

impl<'a> ConfigurationDescriptor<'a> {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new(
        mut head: ConfigurationDescriptorHeader,
        tail: &'a [InterfaceDescriptor],
    ) -> Self {
        head.bLength = size_of::<ConfigurationDescriptorHeader>() as u8;
        head.bNumInterfaces = tail.len() as u8;

        Self { head, tail }
    }

    /// Calculate and update the descriptor total length field
    pub fn set_total_length(&mut self) -> usize {
        let total_length = self.iter().count();
        if let Ok(total_length) = u16::try_from(total_length) {
            self.head.wTotalLength = total_length;
        } else {
            log::warn!(
                "Configuration descriptor is too long. Truncating to {} bytes.",
                u16::MAX
            );
            self.head.wTotalLength = u16::MAX;
        }

        total_length
    }

    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&self) -> ConfigurationDescriptorIterator {
        ConfigurationDescriptorIterator::new(self)
    }
}

/// USB configuration descriptor iterator
pub struct ConfigurationDescriptorIterator<'a> {
    chain: iter::Chain<slice::Iter<'a, u8>, ConfigurationDescriptorTailIterator<'a>>,
}

impl<'a> ConfigurationDescriptorIterator<'a> {
    #[must_use]
    pub fn new(descriptor: &'a ConfigurationDescriptor) -> Self {
        let head_iter: slice::Iter<'a, u8> = descriptor.head.as_iter();
        let tail_iter: ConfigurationDescriptorTailIterator = descriptor
            .tail
            .iter()
            .flat_map(&|x: &'a InterfaceDescriptor| x.iter());
        let chain: iter::Chain<slice::Iter<'a, u8>, ConfigurationDescriptorTailIterator<'a>> =
            head_iter.chain(tail_iter);

        Self { chain }
    }
}

impl<'a> Iterator for ConfigurationDescriptorIterator<'a> {
    type Item = &'a u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.chain.next()
    }
}

// type aliases for sanity
pub type InterfaceDescriptorIterator<'a> =
    CompositeIterator<'a, InterfaceDescriptorHeader, EndpointDescriptor>;
pub type ConfigurationDescriptorTailIterator<'a> = iter::FlatMap<
    slice::Iter<'a, InterfaceDescriptor<'a>>,
    InterfaceDescriptorIterator<'a>,
    &'a dyn Fn(&'a InterfaceDescriptor<'a>) -> InterfaceDescriptorIterator<'a>,
>;

// - InterfaceDescriptor ------------------------------------------------------

/// USB interface descriptor header
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy)]
#[repr(C, packed)]
pub struct InterfaceDescriptorHeader {
    pub bLength: u8,         // 9
    pub bDescriptorType: u8, // 4 = Interface
    pub iInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}

impl AsByteSliceIterator for InterfaceDescriptorHeader {}

impl InterfaceDescriptorHeader {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new() -> Self {
        Self {
            bLength: size_of::<Self>() as u8,
            bDescriptorType: DescriptorType::Interface as u8,
            iInterfaceNumber: 0,
            bAlternateSetting: 0,
            bNumEndpoints: 0,
            bInterfaceClass: 0,
            bInterfaceSubClass: 0,
            bInterfaceProtocol: 0,
            iInterface: 0,
        }
    }
}

/// USB interface descriptor
pub struct InterfaceDescriptor<'a> {
    head: InterfaceDescriptorHeader,
    tail: &'a [EndpointDescriptor],
}

impl<'a> InterfaceDescriptor<'a> {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new(mut head: InterfaceDescriptorHeader, tail: &'a [EndpointDescriptor]) -> Self {
        head.bLength = size_of::<InterfaceDescriptorHeader>() as u8;
        head.bNumEndpoints = tail.len() as u8;
        Self { head, tail }
    }

    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&'a self) -> CompositeIterator<'a, InterfaceDescriptorHeader, EndpointDescriptor> {
        let iter = CompositeIterator::new(&self.head, self.tail);
        iter
    }
}

// - EndpointDescriptor -------------------------------------------------------

/// USB endpoint descriptor
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy)]
#[repr(C, packed)]
pub struct EndpointDescriptor {
    pub bLength: u8,         // 7
    pub bDescriptorType: u8, // 5 = Endpoint
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
}

impl AsByteSliceIterator for EndpointDescriptor {}

impl EndpointDescriptor {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new() -> Self {
        Self {
            bLength: size_of::<Self>() as u8,
            bDescriptorType: DescriptorType::Endpoint as u8,
            bEndpointAddress: 0,
            bmAttributes: 0,
            wMaxPacketSize: 0,
            bInterval: 0,
        }
    }
}

impl Default for EndpointDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// - StringDescriptorZero -----------------------------------------------------

/// USB string descriptor language id
#[derive(AsBytes, Copy, Clone, Debug)]
#[repr(u16)]
pub enum LanguageId {
    EnglishUnitedStates = 0x0409,
    EnglishUnitedKingdom = 0x0809,
    EnglishCanadian = 0x1009,
    EnglishSouthAfrica = 0x1c09,
}

impl AsByteSliceIterator for LanguageId {}

/// USB string zero descriptor
#[derive(Clone, Copy)]
pub struct StringDescriptorZero<'a> {
    head: StringDescriptorHeader,
    tail: &'a [LanguageId],
}

impl<'a> StringDescriptorZero<'a> {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new(language_ids: &'a [LanguageId]) -> Self {
        let head_length = size_of::<StringDescriptorHeader>();
        let tail_length = language_ids.len() * size_of::<LanguageId>();
        Self {
            head: StringDescriptorHeader {
                bLength: (head_length + tail_length) as u8,
                bDescriptorType: DescriptorType::String as u8,
            },
            tail: language_ids,
        }
    }

    #[must_use]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&'a self) -> CompositeIterator<'a, StringDescriptorHeader, LanguageId> {
        let iter = CompositeIterator::new(&self.head, self.tail);
        iter
    }
}

// - StringDescriptor ---------------------------------------------------------

/// USB string zero descriptor header
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct StringDescriptorHeader {
    pub bLength: u8,
    pub bDescriptorType: u8, // 3 = String
}

impl StringDescriptorHeader {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bLength: 0,
            bDescriptorType: DescriptorType::String as u8,
        }
    }
}

impl AsByteSliceIterator for StringDescriptorHeader {}

/// USB String Descriptor
#[derive(Clone, Copy)]
pub struct StringDescriptor<'a> {
    pub head: StringDescriptorHeader,
    pub tail: &'a str,
}

impl<'a> StringDescriptor<'a> {
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn new(string: &'a str) -> Self {
        let head_length = size_of::<StringDescriptorHeader>();
        // TODO this may not be accurate
        let tail_length = string.len() * 2;

        Self {
            head: StringDescriptorHeader {
                bLength: (head_length + tail_length) as u8,
                bDescriptorType: DescriptorType::String as u8,
            },
            tail: string,
        }
    }
}

impl<'a> StringDescriptor<'a> {
    /// Returns an iterator to the descriptor
    #[allow(clippy::cloned_instead_of_copied)]
    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&'a self) -> StringDescriptorIterator<'a> {
        let head_iter: slice::Iter<'a, u8> = self.head.as_iter();

        // TODO USB string descriptors can be a maximum of 126 characters
        let tail_iter: Utf16ByteIterator = Utf16ByteIterator::new(self.tail.encode_utf16());

        head_iter.cloned().chain(tail_iter)
    }
}

pub type StringDescriptorIterator<'a> =
    iter::Chain<iter::Cloned<slice::Iter<'a, u8>>, Utf16ByteIterator<'a>>;

// - Utf16ByteIterator --------------------------------------------------------

#[derive(Clone)]
pub struct Utf16ByteIterator<'a> {
    encode_utf16: core::str::EncodeUtf16<'a>,
    byte: Option<u8>,
}

impl<'a> Utf16ByteIterator<'a> {
    #[must_use]
    pub fn new(encode_utf16: core::str::EncodeUtf16<'a>) -> Self {
        Self {
            encode_utf16,
            byte: None,
        }
    }
}

impl<'a> Iterator for Utf16ByteIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match self.byte {
            Some(_) => self.byte.take(),
            None => match self.encode_utf16.next() {
                Some(unicode_char) => {
                    let bytes: [u8; 2] = unicode_char.to_le_bytes();
                    self.byte = Some(bytes[1]);
                    Some(bytes[0])
                }
                None => None,
            },
        }
    }
}

// - CompositeIterator --------------------------------------------------------

type HeadIterator<'a> = slice::Iter<'a, u8>;
type TailIterator<'a, T> = iter::FlatMap<
    slice::Iter<'a, T>,
    slice::Iter<'a, u8>,
    &'a dyn Fn(&'a T) -> slice::Iter<'a, u8>,
>;
type CompositeChain<'a, T> = iter::Chain<slice::Iter<'a, u8>, TailIterator<'a, T>>;

pub struct CompositeIterator<'a, H, T> {
    chain: CompositeChain<'a, T>,
    _marker: PhantomData<H>,
}

impl<'a, H, T> CompositeIterator<'a, H, T>
where
    H: AsByteSliceIterator + 'a,
    T: AsByteSliceIterator + 'a,
{
    pub fn new(head: &'a H, tail: &'a [T]) -> Self {
        let head_iter: HeadIterator<'a> = head.as_iter();
        let tail_iter: TailIterator<'a, T> = tail.iter().flat_map(&|x: &'a T| x.as_iter());
        let chain: CompositeChain<'a, T> = head_iter.chain(tail_iter);
        Self {
            chain,
            _marker: PhantomData,
        }
    }
}

impl<'a, H, T> Iterator for CompositeIterator<'a, H, T> {
    type Item = &'a u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.chain.next()
    }
}
