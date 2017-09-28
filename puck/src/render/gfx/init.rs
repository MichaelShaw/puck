use glutin;
use gfx_window_glutin;
use gfx_device_gl;
use gfx;
use gfx::Factory;
use {Dimensions, Input, FileResources, PuckResult};

use super::{Renderer, ColorFormat, DepthFormat};

pub fn get_dimensions(window: &glutin::GlWindow) -> Dimensions { // make this optional at some point
    Dimensions {
        pixels: window.get_inner_size_pixels().unwrap_or((100, 100)),
        points: window.get_inner_size_points().unwrap_or((100, 100)),
    }
}

pub type OpenGLResources = gfx_device_gl::Resources;
pub type OpenGLRenderer = Renderer<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer, gfx_device_gl::Factory, gfx_device_gl::Device>;

pub fn construct_opengl_renderer(file_resources: FileResources, dimensions: (u32, u32), vsync: bool, window_name: &str) -> PuckResult<OpenGLRenderer> {
    let (width, height) = dimensions;
    //    println!("pre events");
    let mut events_loop = glutin::EventsLoop::new();
    let window_config = glutin::WindowBuilder::new()
        .with_title(window_name.to_string())
        .with_dimensions(width, height);
    use glutin::{GlRequest, Api};
    let context = glutin::ContextBuilder::new()
        .with_srgb(false)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true);


    //    context = 4;

    //    println!("pre build");
    let (window, mut device, mut factory, mut main_color, mut main_depth) = gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_config, context, &events_loop);

    //    println!("post build");
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    use gfx::texture;
    //    println!("post encoder");
    let sampler_info = texture::SamplerInfo::new(
        texture::FilterMethod::Scale,
        texture::WrapMode::Clamp,
    );

    let sampler = factory.create_sampler(sampler_info);

    let dimensions = get_dimensions(&window);

    let ui_layers = 16;
    let ui_size = 1024;

//    let ui_store_dimensions = TextureArrayDimensions {
//        width: 1024,
//        height: 1024,
//        layers: ui_layers,
//    };

//    let kind = texture_kind_for(&ui_store_dimensions);
    let bind = gfx::SHADER_RESOURCE;
    let cty = gfx::format::ChannelType::Unorm;
//    let ui_tex = factory.create_texture(kind, 1, bind, gfx::memory::Usage::Dynamic, Some(cty)).map_err(PuckError::TextureCreationError)?;
//    let ui_tex_view = factory.view_texture_as_shader_resource::<Srgba8>(&ui_tex, (0, 0), gfx::format::Swizzle::new()).map_err(JamError::ResourceViewError)?;

    // go through the font directory


//    let fonts = load_fonts_in_path(file_resources.font_directory.path.as_path())?;

    //    println!("ok how many loaded fonts -> {:?}", fonts.len());

    Ok(Renderer {
        file_resources,
        window,
        events_loop,
        device,
        factory,
        screen_colour_target: main_color,
        screen_depth_target: main_depth,
        encoder: encoder,
        texture: None,
        sampler,
        pipelines: None,
        dimensions,
        input: Input::default(),
    })
}