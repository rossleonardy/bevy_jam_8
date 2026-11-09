import bevy_pbr::render::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
};

@if(PREPASS_PIPELINE)
import bevy_pbr::{
    prepass::io::{VertexOutput, FragmentOutput},
    deferred::functions::deferred_output,
};
@else
import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
};

struct PlayerExtension {
    end_color: vec4<f32>, // 16 bytes, so no WebGL2 padding needed
}


@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> player: PlayerExtension;
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var mask_texture: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var mask_sampler: sampler;

const emissive_strength: f32 = 1.0;
const START_COLOR: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    let t = clamp(textureSample(mask_texture, mask_sampler, in.uv).r, 0.0, 1.0);
    let ramp = mix(START_COLOR, player.end_color.rgb, t);

    pbr_input.material.base_color = vec4<f32>(ramp, pbr_input.material.base_color.a);

    pbr_input.material.emissive = vec4<f32>(player.end_color.rgb * pow(t, 3.0) * 2.0, pbr_input.material.emissive.a);

    pbr_input.material.base_color = alpha_discard(
        pbr_input.material.flags,
        pbr_input.material.alpha_cutoff,
        pbr_input.material.base_color,
    );

    var out: FragmentOutput;
@if(PREPASS_PIPELINE) {
    out = deferred_output(in, pbr_input);
} @else {
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
}
    return out;
}
