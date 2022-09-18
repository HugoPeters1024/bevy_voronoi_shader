use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType}, sprite::{Material2dPlugin, MaterialMesh2dBundle, Material2d},
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            canvas: Some("#shader_demo".into()),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .add_system(tick)
        .run();
}

struct MatResource {
    handle: Handle<CustomMaterial>,
    velocities: [Vec2; 100],
}

fn setup(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    asset_server.watch_for_changes().unwrap();
    let window = windows.get_primary_mut().unwrap();

    let quad = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    ))));

    let mut points = [Vec4::ZERO; 100];
    for point in points.iter_mut() {
        point.x = rand::random::<f32>();
        point.y = rand::random::<f32>();
    }

    let mut velocities = [Vec2::ZERO; 100];
    for v  in velocities.iter_mut() {
        v.x = rand::random::<f32>() * 2.0 - 1.0;
        v.y = rand::random::<f32>() * 2.0 - 1.0;
        *v = v.normalize();
        v.x *= 0.002;
        v.y *= 0.002;
    }

    let material = materials.add(CustomMaterial {
        points,
    });

    commands.insert_resource(MatResource {
        handle: material.clone(),
        velocities,
    });

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: quad.into(),
            material,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            ..default()
        });

    // camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn tick(
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut mat_res: ResMut<MatResource>) {
    if let Some(mat) = materials.get_mut(&mat_res.handle) {
        for (p, v) in mat.points.iter_mut().zip(mat_res.velocities.iter_mut()) {
            p.x = p.x + v.x;
            if p.x < 0.0 { p.x = 0.0; v.x *= -1.0; }
            if p.x > 1.0 { p.x = 1.0; v.x *= -1.0; }
            p.y = p.y + v.y;
            if p.y < 0.0 { p.y = 0.0; v.y *= -1.0; }
            if p.y > 1.0 { p.y = 1.0; v.y *= -1.0; }
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone, ShaderType)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    points: [Vec4; 100],
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.frag".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/custom_material.vert".into()
    }

    fn specialize(
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayout,
        key: bevy::sprite::Material2dKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }


}
