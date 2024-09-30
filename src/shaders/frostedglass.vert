#version 450 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;

out vec2 Texture;

void main(){
    Texture = aTexCoords;

    vec4 position = vec4(aPos * 1.5, 1.0) ;
    gl_Position = position;
}