use hal::{adapter::Adapter, format as f, image, window};
use crate::{native, Backend as B, GlContainer, PhysicalDevice, QueueFamily};


struct PixelFormat {
    color_bits: u32,
    alpha_bits: u32,
    srgb: bool,
    double_buffer: bool,
    multisampling: Option<u32>,
}

#[derive(Clone, Copy, Debug)]
pub struct Window;

impl Window {
    fn get_pixel_format(&self) -> PixelFormat {
        PixelFormat {
            color_bits: 24,
            alpha_bits: 8,
            srgb: false,
            double_buffer: true,
            multisampling: None,
        }
    }

    pub fn get_window_extent(&self) -> image::Extent {
        image::Extent {
            width: 640,
            height: 480,
            depth: 1,
        }
    }

    pub fn get_hidpi_factor(&self) -> f64 {
        1.0
    }

    pub fn resize<T>(&self, parameter: T) {}
}

#[derive(Clone, Debug)]
pub struct Swapchain {
    pub(crate) extent: window::Extent2D,
    pub(crate) fbos: Vec<native::RawFrameBuffer>,
}

impl window::Swapchain<B> for Swapchain {
    unsafe fn acquire_image(
        &mut self,
        _timeout_ns: u64,
        _semaphore: Option<&native::Semaphore>,
        _fence: Option<&native::Fence>,
    ) -> Result<(window::SwapImageIndex, Option<window::Suboptimal>), window::AcquireError> {
        // TODO: sync
        Ok((0, None))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Surface;

impl Surface {
    pub fn from_window(_window: &Window) -> Self {
        Surface
    }

    fn swapchain_formats(&self) -> Vec<f::Format> {
        let pixel_format = Window.get_pixel_format();
        let color_bits = pixel_format.color_bits;
        let alpha_bits = pixel_format.alpha_bits;
        let srgb = pixel_format.srgb;

        // TODO: expose more formats
        match (color_bits, alpha_bits, srgb) {
            (24, 8, true) => vec![f::Format::Rgba8Srgb, f::Format::Bgra8Srgb],
            (24, 8, false) => vec![f::Format::Rgba8Unorm, f::Format::Bgra8Unorm],
            _ => vec![],
        }
    }
}

impl window::Surface<B> for Surface {
    fn compatibility(
        &self,
        _: &PhysicalDevice,
    ) -> (
        window::SurfaceCapabilities,
        Option<Vec<f::Format>>,
        Vec<window::PresentMode>,
    ) {
        let ex = Window.get_window_extent();
        let extent = window::Extent2D::from(ex);

        let caps = window::SurfaceCapabilities {
            image_count: if Window.get_pixel_format().double_buffer {
                2 ..= 2
            } else {
                1 ..= 1
            },
            current_extent: Some(extent),
            extents: extent ..= extent,
            max_image_layers: 1,
            usage: image::Usage::COLOR_ATTACHMENT | image::Usage::TRANSFER_SRC,
            composite_alpha: window::CompositeAlpha::OPAQUE, //TODO
        };
        let present_modes = vec![
            window::PresentMode::Fifo, //TODO
        ];

        (caps, Some(self.swapchain_formats()), present_modes)
    }

    fn supports_queue_family(&self, _: &QueueFamily) -> bool {
        true
    }
}

impl hal::Instance for Surface {
    type Backend = B;
    fn enumerate_adapters(&self) -> Vec<Adapter<B>> {
        let adapter = PhysicalDevice::new_adapter((), GlContainer::from_new_canvas()); // TODO: Move to `self` like native/window
        vec![adapter]
    }
}
