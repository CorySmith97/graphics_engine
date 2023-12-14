@header use crate::math as m
@ctype mat4 m::Mat4
@ctype vec4 m::Vec4
@ctype vec3 m::Vec3
@ctype vec2 m::Vec2


@vs vs 

uniform vs_params {
    mat4 mvp;
};

in vec4 pos;
in vec4 color;
in vec2 texcoords0;

out vec4 fg_color;
out vec2 uv;

void main() {
  gl_Position = mvp * pos;
  fg_color = color;
  uv = texcoords0;
}
@end

@fs fs
uniform texture2D tex;
uniform sampler smp;

in vec4 fg_color;
in vec2 uv;

out vec4 frag_color;

void main() {
    frag_color = texture(sampler2D(tex, smp), uv) * fg_color;
}
@end

@program shader vs fs
