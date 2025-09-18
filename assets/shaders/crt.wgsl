// CRT monitor effect shader
// Creates retro CRT scanlines, vignette, and subtle color distortion

// Import Bevy's fullscreen vertex shader
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct CrtSettings {
    time: f32,
    scanline_intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned
    _webgl2_padding: vec2<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: CrtSettings;

fn remap(value: f32, input_min: f32, input_max: f32, output_min: f32, output_max: f32) -> f32 {
    let t = (value - input_min) / (input_max - input_min);
    return mix(output_min, output_max, t);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Sample the screen texture
    var color = textureSample(screen_texture, texture_sampler, in.uv).rgb;

    // Scanline effect - two layers with different frequencies and phases
    let scanline_1 = remap(
        sin(in.uv.y * 400.0 + settings.time * 10.0),
        -1.0,
        1.0,
        0.9,
        1.0
    );

    let scanline_2 = remap(
        sin(in.uv.y * 200.0 - settings.time * 20.0),
        -1.0,
        1.0,
        0.95,
        1.0
    );

    // Apply scanline intensity
    let scanline_effect = mix(1.0, scanline_1 * scanline_2, settings.scanline_intensity);

    // Apply scanlines to color
    color = color * scanline_effect;

    // Vignette effect - darkens edges for CRT look
    let center_distance = distance(in.uv, vec2<f32>(0.5, 0.5));
    let vignette = 1.0 - center_distance * 0.3;
    color = color * vignette;

    // Subtle color shift for vintage CRT look
    let shift_amount = 0.002;
    let red = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(shift_amount, 0.0)).r;
    let green = textureSample(screen_texture, texture_sampler, in.uv).g;
    let blue = textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-shift_amount, 0.0)).b;

    // Mix the color-shifted version with original
    let shifted_color = vec3<f32>(red, green, blue) * scanline_effect * vignette;
    color = mix(color, shifted_color, 0.1);

    return vec4<f32>(color, 1.0);
}