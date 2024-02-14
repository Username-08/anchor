use std::{
    collections::BTreeMap, env::var, fs::File, io::Read,
    time::Duration,
};

use smithay::{
    backend::renderer::{
        element::{
            surface::{
                render_elements_from_surface_tree,
                WaylandSurfaceRenderElement,
            },
            texture::{TextureBuffer, TextureRenderElement},
            AsRenderElements, Kind,
        },
        ImportAll, ImportMem, Renderer, Texture,
    },
    input::pointer::CursorImageStatus,
    render_elements,
    utils::{Clock, Monotonic},
};
use xcursor::{parser::parse_xcursor, CursorTheme};

pub struct PointerElement<T: Texture> {
    default: BTreeMap<u64, TextureBuffer<T>>,
    total_delay: u64,
    current_delay: u64,
    status: CursorImageStatus,
}

impl<T: Texture> PointerElement<T> {
    pub fn new<R>(renderer: &mut R) -> Self
    where
        R: Renderer<TextureId = T> + ImportMem,
    {
        let theme =
            var("XCURSOR_THEME").ok().unwrap_or("default".into());

        let size = var("XCURSOR_SIZE")
            .ok()
            .and_then(|s| s.parse::<i32>().ok())
            .unwrap_or(24);

        let cursor_theme = CursorTheme::load(&theme);
        let cursor_path = cursor_theme.load_icon("left_ptr").unwrap();

        let mut cursor_file = File::open(&cursor_path).unwrap();
        let mut cursor_data = vec![];
        cursor_file.read_to_end(&mut cursor_data).unwrap();

        let cursor_images = parse_xcursor(&cursor_data)
            .unwrap()
            .into_iter()
            .filter(move |image| {
                image.width == size as u32
                    && image.height == size as u32
            });

        let mut default = BTreeMap::new();

        let mut total_delay = 0;
        for image in cursor_images {
            total_delay += image.delay as u64;

            let texture = renderer
                .import_memory(
                    image.pixels_rgba.as_slice(),
                    smithay::backend::allocator::Fourcc::Rgba8888,
                    (size, size).into(),
                    false,
                )
                .unwrap();

            let texture_buffer = TextureBuffer::from_texture(
                renderer,
                texture,
                1,
                smithay::utils::Transform::Normal,
                None,
            );
            default.insert(total_delay, texture_buffer);
        }

        Self {
            default,
            total_delay,
            current_delay: 0,
            status: CursorImageStatus::default_named(),
        }
    }

    pub fn set_current_delay(&mut self, clock: &Clock<Monotonic>) {
        let current_duration = Duration::from(clock.now());
        self.current_delay =
            self.total_delay % current_duration.as_millis() as u64;
    }

    pub fn set_status(&mut self, status: CursorImageStatus) {
        self.status = status;
    }
}

render_elements! {
    pub PointerRenderElement<R> where
        R: ImportAll;
    Surface = WaylandSurfaceRenderElement<R>,
    Texture = TextureRenderElement<<R as Renderer>::TextureId>,
}

impl<T: Texture + Clone + 'static, R> AsRenderElements<R>
    for PointerRenderElement<T>
where
    R: Renderer<TextureId = T> + ImportAll,
{
    type RenderElement = PointerRenderElement<R>;

    fn render_elements<C: From<Self::RenderElement>>(
        &self,
        renderer: &mut R,
        location: smithay::utils::Point<
            i32,
            smithay::utils::Physical,
        >,
        scale: smithay::utils::Scale<f64>,
        alpha: f32,
    ) -> Vec<C>
    where
        C: From<PointerRenderElement<R>>,
    {
        match &self.status {
            CursorImageStatus::Hidden => vec![],
            CursorImageStatus::Surface(surface) => {
                render_elements_from_surface_tree(
                    renderer,
                    surface,
                    location,
                    scale,
                    alpha,
                    Kind::Cursor,
                )
                .into_iter()
                .map(C::from)
                .collect()
            }
            CursorImageStatus::Named(name) => {}
        }
    }
}