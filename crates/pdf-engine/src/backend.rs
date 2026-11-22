//! PDF backend trait abstraction.
//! Defines the operations the coordinator needs from a PDF engine, allowing
//! the backend to be swapped without changing the coordinator or subsystems.
use viewer_core::PageSize;

/// A page handle returned by the backend.
pub trait PageBackend {
    /// Returns the page's width and height in PDF points.
    fn bounds(&self) -> Result<PageSize, String>;

    /// Builds a display list for this page.
    fn to_display_list(&self, build: bool) -> Result<Box<dyn DisplayListBackend>, String>;
}

/// A display list handle.
pub trait DisplayListBackend: Send + Sync {
    /// Returns the display list's bounding box as (x0, y0, x1, y1).
    fn bounds(&self) -> Result<(f32, f32, f32, f32), String>;

    /// Renders the display list to a pixel tile.
    fn render(
        &self,
        width: u32,
        height: u32,
        rotation: u32,
        scale_milli: u32,
        x: f32,
        y: f32,
    ) -> Result<viewer_core::PixelTile, String>;
}

/// A document handle returned by the backend.
pub trait DocumentBackend {
    /// Loads a page by index.
    fn load_render_page(&self, page: i32) -> Result<Box<dyn PageBackend>, String>;
}

/// MuPDF implementation of the backend traits.
pub mod mupdf {
    use super::*;
    use ::mupdf::pdf::PdfDocument;
    use ::mupdf::{Colorspace, Device, DisplayList, IRect, Matrix, Page, Pixmap, Rect};

    impl DocumentBackend for PdfDocument {
        fn load_render_page(&self, page: i32) -> Result<Box<dyn PageBackend>, String> {
            let page = self.load_page(page).map_err(|e| e.to_string())?;
            Ok(Box::new(MupdfPage { page }))
        }
    }

    pub struct MupdfPage {
        page: Page,
    }

    impl MupdfPage {}

    impl PageBackend for MupdfPage {
        fn bounds(&self) -> Result<PageSize, String> {
            let b = self.page.bounds().map_err(|e| e.to_string())?;
            Ok(PageSize::new(b.width(), b.height()))
        }

        fn to_display_list(&self, build: bool) -> Result<Box<dyn DisplayListBackend>, String> {
            let list = self
                .page
                .to_display_list(build)
                .map_err(|e| e.to_string())?;
            Ok(Box::new(MupdfDisplayList { list }))
        }
    }

    pub struct MupdfDisplayList {
        list: DisplayList,
    }

    impl MupdfDisplayList {
        #[cfg(test)]
        pub(crate) fn new(list: DisplayList) -> Self {
            Self { list }
        }
    }

    impl DisplayListBackend for MupdfDisplayList {
        fn bounds(&self) -> Result<(f32, f32, f32, f32), String> {
            let b = self.list.bounds();
            Ok((b.x0, b.y0, b.x1, b.y1))
        }

        fn render(
            &self,
            width: u32,
            height: u32,
            _rotation: u32,
            scale_milli: u32,
            x: f32,
            y: f32,
        ) -> Result<viewer_core::PixelTile, String> {
            let (x0, y0, _, _) = self.bounds()?;
            let cs = Colorspace::device_rgb();
            let mut pixmap = Pixmap::new_with_w_h(&cs, width as i32, height as i32, false)
                .map_err(|e| e.to_string())?;
            pixmap
                .clear_with(crate::constants::CLEAR_COLOR.into())
                .map_err(|e| e.to_string())?;
            let device = Device::from_pixmap_with_clip(
                &pixmap,
                IRect::new(0, 0, width as i32, height as i32),
            )
            .map_err(|e| e.to_string())?;
            let scale = scale_milli as f32 / 1000.0;
            let matrix = Matrix::new(scale, 0.0, 0.0, scale, -x0 * scale - x, -y0 * scale - y);
            self.list
                .run(
                    &device,
                    &matrix,
                    Rect::new(0.0, 0.0, width as f32, height as f32),
                )
                .map_err(|e| e.to_string())?;
            drop(device);
            let stride = pixmap.stride() as usize;
            let mut rgba = Vec::with_capacity((width * height * 4) as usize);
            for row in pixmap.samples().chunks(stride).take(height as usize) {
                for pixel in row[..width as usize * 3].as_chunks::<3>().0 {
                    rgba.extend_from_slice(&[
                        pixel[0],
                        pixel[1],
                        pixel[2],
                        crate::constants::ALPHA_OPAQUE,
                    ]);
                }
            }
            Ok(viewer_core::PixelTile {
                width,
                height,
                rgba,
            })
        }
    }
}
