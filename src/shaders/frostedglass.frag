#version 450 core
out vec4 FragColor;

in vec2 Texture;

uniform float millis;
uniform sampler2D background;
uniform float blur_radius;

vec4 applyBlur(vec2 uv, float radius) {
    vec4 color = vec4(0.0);
    int samples = 10;
    for (int i = -samples; i <= samples; i++){
        for (int j = -samples; j <= samples; j++){
            vec2 offset = vec2(float(i), float(j)) * radius / vec2(1000.0, 800.0);
            color += texture(background, uv + offset);
        }
    }

    return color / pow(2.0 * samples + 1.0, 2.0);
}

void main() {
    vec2 uv = Texture;
    uv.y = 1.0 - uv.y;
    // vec4 col = texture(background, uv);
    // float avg = (col.r + col.g + col.b) / 3.0;
    vec4 background_color = applyBlur(uv, blur_radius);
    vec4 glass_tint = vec4(1.0, 1.0, 1.0, 0.5);
    FragColor = mix(background_color, glass_tint, 0.5);
}
