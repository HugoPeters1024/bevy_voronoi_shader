#version 450
layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 1, binding = 0) uniform Points {
  vec4 data[100];
} pts;

uint rand_xorshift(uint seed)
{
    // Xorshift algorithm from George Marsaglia's paper
    seed ^= (seed << 13);
    seed ^= (seed >> 17);
    seed ^= (seed << 5);
    return seed;
}

float rand(inout uint seed)
{
    seed = rand_xorshift(seed);
    return seed * 2.3283064365387e-10f;
}

vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
  float min_dis = 100000;
  uint min_id = 0;
  for(uint i=0; i<100; i++) {
    float dis = length(pts.data[i].xy - v_Uv);
    if (dis < min_dis) {
      min_dis = dis;
      min_id = i;
    }
  }

  float intensity = pow(1.0 - min_dis, 10.5);

  uint seed = rand_xorshift(min_id);
  float hue = rand(seed);
  float saturation = 0.4 + 0.6 * rand(seed);
  float value = 0.2 + 0.8 * rand(seed);
  vec3 color = intensity * hsv2rgb(vec3(hue, saturation, value));



  o_Target = vec4(color, 1.0);
}
