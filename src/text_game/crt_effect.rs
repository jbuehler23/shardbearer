//! CRT monitor effect post-processing shader for the cyberpunk terminal game.
//! This creates a retro CRT monitor effect with scanlines and vignette.

use bevy::{
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        FullscreenShader,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphContext, RenderGraphExt, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, RenderDevice},
        view::ViewTarget,
        RenderApp, RenderStartup,
    },
};

/// Path to the CRT shader file
const SHADER_ASSET_PATH: &str = "shaders/crt.wgsl";

/// Plugin that adds CRT post-processing effect
pub struct CrtEffectPlugin;

impl Plugin for CrtEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // Extract the settings component from main world to render world every frame
            ExtractComponentPlugin::<CrtSettings>::default(),
            // Prepare uniform buffer for GPU
            UniformComponentPlugin::<CrtSettings>::default(),
        ));

        // Get the render app
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        // Initialize the pipeline on startup
        render_app.add_systems(RenderStartup, init_crt_pipeline);

        // Add the CRT node to the render graph for 2D
        render_app
            .add_render_graph_node::<ViewNodeRunner<CrtNode>>(Core2d, CrtLabel)
            .add_render_graph_edges(
                Core2d,
                (
                    Node2d::Tonemapping,
                    CrtLabel,
                    Node2d::EndMainPassPostProcessing,
                ),
            );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct CrtLabel;

/// The CRT post-processing node
#[derive(Default)]
struct CrtNode;

impl ViewNode for CrtNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static CrtSettings,
        &'static DynamicUniformIndex<CrtSettings>,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _crt_settings, settings_index): QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        // Get the pipeline resource
        let crt_pipeline = world.resource::<CrtPipeline>();

        // Get the pipeline from cache
        let pipeline_cache = world.resource::<PipelineCache>();
        let Some(pipeline) = pipeline_cache.get_render_pipeline(crt_pipeline.pipeline_id) else {
            return Ok(());
        };

        // Get the settings uniform binding
        let settings_uniforms = world.resource::<ComponentUniforms<CrtSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        // Get source and destination textures
        let post_process = view_target.post_process_write();

        // Create bind group for this frame
        let bind_group = render_context.render_device().create_bind_group(
            "crt_bind_group",
            &crt_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &crt_pipeline.sampler,
                settings_binding.clone(),
            )),
        );

        // Create render pass
        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("crt_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                depth_slice: None,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Render fullscreen triangle
        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[settings_index.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

/// Resource containing the CRT pipeline data
#[derive(Resource)]
struct CrtPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

/// Initialize the CRT pipeline
fn init_crt_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
    pipeline_cache: Res<PipelineCache>,
) {
    // Create bind group layout
    let layout = render_device.create_bind_group_layout(
        "crt_bind_group_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                // Screen texture
                texture_2d(TextureSampleType::Float { filterable: true }),
                // Texture sampler
                sampler(SamplerBindingType::Filtering),
                // CRT settings uniform
                uniform_buffer::<CrtSettings>(true),
            ),
        ),
    );

    // Create sampler
    let sampler = render_device.create_sampler(&SamplerDescriptor::default());

    // Load shader
    let shader = asset_server.load(SHADER_ASSET_PATH);

    // Create vertex state (fullscreen triangle)
    let vertex_state = fullscreen_shader.to_vertex_state();

    // Add shader defines for WebGL2 compatibility
    let shader_defs = {
        let mut defs = Vec::new();
        #[cfg(all(feature = "webgl2", target_arch = "wasm32", not(feature = "webgpu")))]
        {
            defs.push("SIXTEEN_BYTE_ALIGNMENT".into());
        }
        defs
    };

    // Queue render pipeline
    let pipeline_id = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("crt_pipeline".into()),
        layout: vec![layout.clone()],
        vertex: vertex_state,
        fragment: Some(FragmentState {
            shader,
            shader_defs,
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::bevy_default(),
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
            ..default()
        }),
        ..default()
    });

    commands.insert_resource(CrtPipeline {
        layout,
        sampler,
        pipeline_id,
    });
}

/// CRT effect settings component
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct CrtSettings {
    pub time: f32,
    pub scanline_intensity: f32,
    // WebGL2 structs must be 16 byte aligned
    #[cfg(all(feature = "webgl2", target_arch = "wasm32", not(feature = "webgpu")))]
    _webgl2_padding: Vec2,
}

impl CrtSettings {
    pub fn new(time: f32, scanline_intensity: f32) -> Self {
        Self {
            time,
            scanline_intensity,
            #[cfg(all(feature = "webgl2", target_arch = "wasm32", not(feature = "webgpu")))]
            _webgl2_padding: Vec2::ZERO,
        }
    }
}