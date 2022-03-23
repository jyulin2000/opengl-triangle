#version 400 core

out vec4 Color;

uniform vec4 ourColor;

void main()
{
    //Color = vec4(IN.Color, 1.0f);
    //Color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    //Color = vec4(IN.Color, 1.0f);
    Color = ourColor;
}