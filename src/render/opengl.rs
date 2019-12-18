//! OpenGL bindings & functions
//!
//! The raw bindings are generated at build time, see build.rs

/// Auto-generated OpenGL bindings from gl_generator
#[allow(clippy::all)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

use crate::{
    atlas::{AtlasBuilder, AtlasRef},
    render::{Renderer, Texture},
};
use glutin::{
    event_loop::EventLoop,
    window::{Fullscreen, Icon, Window, WindowBuilder},
    ContextWrapper, PossiblyCurrent, {Api, ContextBuilder, GlProfile, GlRequest},
};
use rect_packer::DensePacker;
use std::ptr;

// OpenGL typedefs
use gl::types::{GLint, GLuint};

pub struct OpenGLRenderer {
    ctx: ContextWrapper<PossiblyCurrent, ()>,
    el: EventLoop<()>,
    window: Window,

    // texture atlases
    atlases_initialized: bool,
    atlas_packers: Vec<DensePacker>,
    atlas_refs: Vec<AtlasRef>, // `Texture` indexes this
}

pub struct OpenGLRendererOptions<'a> {
    pub title: &'a str,
    pub size: (u32, u32),
    pub icon: Option<(Vec<u8>, u32, u32)>,
    pub resizable: bool,
    pub on_top: bool,
    pub decorations: bool,
    pub fullscreen: bool,
    pub vsync: bool,
}

impl OpenGLRenderer {
    pub fn new(options: OpenGLRendererOptions) -> Result<Self, String> {
        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title(options.title)
            .with_window_icon(options.icon.and_then(|(data, w, h)| Icon::from_rgba(data, w, h).ok()))
            .with_inner_size(options.size.into())
            .with_resizable(options.resizable)
            .with_always_on_top(options.on_top)
            .with_decorations(options.decorations)
            .with_visible(false)
            .with_fullscreen(if options.fullscreen {
                // TODO: Allow overriding primary monitor
                Some(Fullscreen::Borderless(el.primary_monitor()))
            } else {
                None
            });

        let ctx = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .with_gl_profile(GlProfile::Core)
            .with_hardware_acceleration(Some(true))
            .with_vsync(options.vsync)
            .build_windowed(wb, &el)
            .map_err(|err| err.to_string())?;

        let (ctx, window) = unsafe { ctx.make_current().map_err(|(_self, err)| err.to_string())?.split() };

        gl::load_with(|s| ctx.get_proc_address(s) as *const _);

        Ok(Self {
            ctx,
            el,
            window,

            atlases_initialized: false,
            atlas_packers: Vec::new(),
            atlas_refs: Vec::new(),
        })
    }
}

impl Renderer for OpenGLRenderer {
    fn max_gpu_texture_size(&self) -> usize {
        unsafe {
            let mut v: GLint = 0;
            gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut v as _);
            v as _
        }
    }

    fn process_atlases(&mut self, atl: AtlasBuilder) -> Vec<Texture> {
        assert!(!self.atlases_initialized, "atlases should be initialized only once");

        let (packers, sprites) = atl.into_inner();

        unsafe {
            let textures: Vec<GLuint> = {
                let mut buf = vec![0 as GLuint; packers.len()];
                gl::GenTextures(buf.len() as _, buf.as_mut_ptr());
                for (tex_id, packer) in buf.iter().copied().zip(&packers) {
                    let (width, height) = packer.size();

                    gl::BindTexture(gl::TEXTURE_2D, tex_id);
                    gl::TexImage2D(
                        gl::TEXTURE_2D,    // target
                        0,                 // level
                        gl::RGBA as _,     // internalformat
                        width as _,        // width
                        height as _,       // height
                        0,                 // border ("must be 0")
                        gl::BGRA,          // format
                        gl::UNSIGNED_BYTE, // type
                        ptr::null(),       // data
                    );
                }
                buf
            };

            // upload textures
            let mut current_texture: GLint = 0;
            for (atl_ref, pixels) in sprites {
                if current_texture != atl_ref.atlas_id as _ {
                    gl::BindTexture(gl::TEXTURE_2D, textures[atl_ref.atlas_id as usize]);
                    current_texture = atl_ref.atlas_id as _;
                }

                gl::TexSubImage2D(
                    gl::TEXTURE_2D,       // target
                    0,                    // level
                    atl_ref.x as _,       // xoffset
                    atl_ref.y as _,       // yoffset
                    atl_ref.w as _,       // width
                    atl_ref.h as _,       // height
                    gl::BGRA,             // format
                    gl::UNSIGNED_BYTE,    // type
                    pixels.as_ptr() as _, // pixels
                );
            }

            // DELETE THIS AFTER ETESTING OK ?? ? ? OK !
            for ((i, texture), packer) in textures.iter().enumerate().zip(packers.iter()) {
                gl::BindTexture(gl::TEXTURE_2D, *texture);
                let w = std::io::BufWriter::new(std::fs::File::create(format!("./atlas{}.png", i)).unwrap());
                let (width, height) = packer.size();
                let mut encoder = png::Encoder::new(w, width as _, height as _);
                encoder.set_color(png::ColorType::RGBA);
                encoder.set_depth(png::BitDepth::Eight);
                let mut writer = encoder.write_header().unwrap();

                let mut buf = vec![0u8; width as usize * height as usize * 4];
                gl::GetTexImage(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    buf.as_mut_ptr() as *mut _,
                );

                writer.write_image_data(&buf).unwrap();
            }

            // delete the textures
            gl::DeleteTextures(textures.len() as _, &textures as *const _ as *mut _);
        }

        println!("LAST OPENGL ERROR: {}", unsafe { gl::GetError() });

        unimplemented!()
    }
}
