#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var bg_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var cloud_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var base_color_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> offset1: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> offset2: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> blur: f32;

/// 在指定 UV 下合成两层背景
fn composite(uv: vec2<f32>) -> vec4<f32> {
    let bg = textureSample(bg_texture, base_color_sampler, uv + vec2(offset1, 0.));
    let cloud = textureSample(cloud_texture, base_color_sampler, uv + vec2(offset2, 0.));
    // 云层 alpha 混合到底层上
    return vec4(bg.rgb * (1.0 - cloud.a) + cloud.rgb * cloud.a, 1.0);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let sigma = max(blur, 0.01);
    let ts = 1.0 / vec2<f32>(textureDimensions(bg_texture));
    let two_sigma2 = 2.0 * sigma * sigma;
    let radius = max(i32(ceil(sigma * 3.0)), 0);

    var color = vec4(0.0);
    var weight_sum = 0.0;

    for (var x = -radius; x <= radius; x++) {
        for (var y = -radius; y <= radius; y++) {
            let uv = mesh.uv + vec2(f32(x), f32(y)) * ts * 0.5;
            let dist2 = f32(x * x + y * y) * 0.25;
            let weight = exp(-dist2 / two_sigma2);

            color += composite(uv) * weight;
            weight_sum += weight;
        }
    }
    return color / weight_sum;
}