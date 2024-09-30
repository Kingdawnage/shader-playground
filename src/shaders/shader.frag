#version 450 core
out vec4 FragColor;

in vec2 Texture;

uniform float millis;
uniform sampler2D background;

void main() {
    vec2 uv = Texture;
    uv.y = 1.0 - uv.y;
    vec4 col = texture2D(background, uv);
    float avg = (col.r + col.g + col.b) / 3.0;
    FragColor = vec4(avg, avg, avg, 1.0);
}
