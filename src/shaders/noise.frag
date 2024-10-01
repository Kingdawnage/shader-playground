#version 450 core
out vec4 FragColor;

in vec2 Texture;

uniform float millis;
uniform vec2 resolution;

vec3 palette(float t) {
    vec3 a = vec3(0.5, 0.5, 0.5);
    vec3 b = vec3(0.5, 0.7, 0.5);
    vec3 c = vec3(1.0, 1.0, 1.0);
    vec3 d = vec3(0.3804, 0.2588, 0.0);


    return a + b * cos(6.28318 * (c * t + d));
}

void main() {
    vec2 uv = Texture - 0.5;
    uv.x *= resolution.x / resolution.y;

    vec2 uv0 = uv;
    vec3 finalColor = vec3(0.0);

    for (float i = 0.0; i < 4.0; i++) {
    uv = fract(uv * 1.5) - 0.5;

    float d = length(uv) * exp(-length(uv) * 1.0);
    vec3 col = palette(length(uv0) + i * 0.4 + millis * 0.4);

    d = sin(d * 15.0 + millis) * 0.1;
    d = abs(d);
    // d = smoothstep(0.0, 0.05, d);
    d = pow(0.01 / d, 1.2);
    finalColor += col * d;
    }

    FragColor = vec4(finalColor, 1.0);
}