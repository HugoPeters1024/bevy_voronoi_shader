struct Points {
  data: array<vec4<f32>, 100>
};

@group(1) @binding(0)
var<uniform> points: Points;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    // Get screen position with coordinates from 0 to 1
    let uv = vec2<f32>(position.x/1280.0, position.y/720.0);

    var min_dis = 100000.0;
    for(var i=0; i<100; i++) {
      let dis = length(points.data[i].xy - uv);
      if dis < min_dis {
        min_dis = dis;
      }
    }

    let color = vec3<f32>(pow(1.0 - min_dis, 8.5));

    return vec4<f32>(color, 1.0);

}
