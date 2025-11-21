use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::matrix::multiply_matrix_vector4;
use crate::fragment::Fragment;

// --- Funciones de Ruido 3D (Value Noise & FBM) ---

fn hash(i: i32, j: i32, k: i32) -> f32 {
    let n = (i as f32 * 73.0 + j as f32 * 53.0 + k as f32 * 47.0).sin() * 10000.0;
    n - n.floor()
}

fn smoothstep(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn value_noise_3d(p: Vector3) -> f32 {
    let pi = Vector3::new(p.x.floor(), p.y.floor(), p.z.floor());
    let pf = Vector3::new(p.x - pi.x, p.y - pi.y, p.z - pi.z);

    let u = smoothstep(pf.x);
    let v = smoothstep(pf.y);
    let w = smoothstep(pf.z);

    let a = lerp(
        hash(pi.x as i32, pi.y as i32, pi.z as i32),
        hash(pi.x as i32 + 1, pi.y as i32, pi.z as i32),
        u,
    );
    let b = lerp(
        hash(pi.x as i32, pi.y as i32 + 1, pi.z as i32),
        hash(pi.x as i32 + 1, pi.y as i32 + 1, pi.z as i32),
        u,
    );
    let c = lerp(
        hash(pi.x as i32, pi.y as i32, pi.z as i32 + 1),
        hash(pi.x as i32 + 1, pi.y as i32, pi.z as i32 + 1),
        u,
    );
    let d = lerp(
        hash(pi.x as i32, pi.y as i32 + 1, pi.z as i32 + 1),
        hash(pi.x as i32 + 1, pi.y as i32 + 1, pi.z as i32 + 1),
        u,
    );

    let e = lerp(a, b, v);
    let f = lerp(c, d, v);

    lerp(e, f, w).clamp(0.0, 1.0)
}

fn fbm_noise(p: Vector3, time: f32) -> f32 {
    let mut total = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;

    for i in 0..5 {
        let p_octave = Vector3::new(p.x * frequency, p.y * frequency, p.z * frequency);
        let offset = Vector3::new(time * 0.4, time * 0.3, time * 0.5);
        let noise_val = value_noise_3d(p_octave + offset);

        total += noise_val * amplitude;
        max_value += amplitude;

        amplitude *= 0.5;
        frequency *= 2.0;

        if i == 1 {
            amplitude *= 1.2;
        }
    }

    total / max_value
}

// --- Vertex Shader ---

fn transform_normal(normal: &Vector3, model_matrix: &Matrix) -> Vector3 {
    let normal_vec4 = Vector4::new(normal.x, normal.y, normal.z, 0.0);
    let transformed_normal_vec4 = multiply_matrix_vector4(model_matrix, &normal_vec4);

    let v = Vector3::new(
        transformed_normal_vec4.x,
        transformed_normal_vec4.y,
        transformed_normal_vec4.z,
    );

    // normalized() devuelve un Vector3 (no in-place)
    v.normalized()
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let time = uniforms.time;
    let original_pos = vertex.position;

    // 1. Desplazamiento usando FBM
    let turbulence = fbm_noise(original_pos * 5.0, time * 0.5);
    let displacement_factor = turbulence.powf(2.0) * 0.1;

    // Aplicar distorsión a lo largo del normal
    let distorted_pos = Vector3::new(
        original_pos.x + vertex.normal.x * displacement_factor,
        original_pos.y + vertex.normal.y * displacement_factor,
        original_pos.z + vertex.normal.z * displacement_factor,
    );

    // 2. Transformaciones (M * V * P)
    let distorted_vec4 = Vector4::new(distorted_pos.x, distorted_pos.y, distorted_pos.z, 1.0);

    let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &distorted_vec4);
    let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);
    let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

    // 3. Perspective Division (NDC)
    let ndc = if clip_position.w != 0.0 {
        Vector3::new(
            clip_position.x / clip_position.w,
            clip_position.y / clip_position.w,
            clip_position.z / clip_position.w,
        )
    } else {
        Vector3::new(clip_position.x, clip_position.y, clip_position.z)
    };

    // 4. Viewport Transformation
    let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
    let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

    let transformed_position = Vector3::new(screen_position.x, screen_position.y, screen_position.z);

    Vertex {
        position: original_pos,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix),
    }
}

// --- Fragment Shader ---

fn color_gradient(t: f32) -> Vector3 {
    let t = t.clamp(0.0, 1.0);

    let core = Vector3::new(1.0, 0.1, 0.0);
    let surface = Vector3::new(1.0, 0.6, 0.2);
    let flare = Vector3::new(1.0, 1.0, 0.7);

    if t < 0.5 {
        let t_norm = t * 2.0;
        lerp_vec3(core, surface, t_norm)
    } else {
        let t_norm = (t - 0.5) * 2.0;
        lerp_vec3(surface, flare, t_norm)
    }
}

fn lerp_vec3(a: Vector3, b: Vector3, t: f32) -> Vector3 {
    Vector3::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t), lerp(a.z, b.z, t))
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let pos_world = fragment.world_position;
    let time = uniforms.time;

    // normal_world: nuevo Vector3 normalizado
    let normal_world = pos_world.normalized();

    // Turbulencia FBM
    let turbulence = fbm_noise(pos_world * 3.5, time * 0.9);

    // Pulsación global
    let pulsation = (time * 0.7).sin() * 0.15 + 0.85;

    // Mapear ruido a intensidad
    let intensity_noise = turbulence.powf(2.0);
    let total_intensity = (intensity_noise * pulsation * 1.5).clamp(0.0, 1.0);
    let base_color = color_gradient(total_intensity);

    // --- RIM (borde) ---
    // normaliza pos_world y calcula dot manualmente (evita confusión de firmas)
    let pos_norm: Vector3 = pos_world.normalized();

    // producto punto calculado manualmente:
    let dot_np = normal_world.x * pos_norm.x
               + normal_world.y * pos_norm.y
               + normal_world.z * pos_norm.z;

    let rim_factor = (1.0 - dot_np.abs()).powf(4.0);
    let rim_intensity = rim_factor * 1.5;

    // Combinar colores
    let emission_multiplier = 4.0;
    let final_color = base_color * emission_multiplier
        + Vector3::new(1.0, 0.8, 0.4) * rim_intensity;

    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}
