use bevy::asset::Assets;
use bevy::ecs::system::SystemParam;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Query, ResMut, Resource, With};
use bevy::render::mesh::Mesh;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::ColorMaterial;
use bevy::transform::components::Transform;
use bevy::window::{PrimaryWindow, Window};
use bevy_vector_shapes::prelude::ShapePainter;
use lyon::path::math::point;
use lyon::path::path::BuilderImpl;
use lyon::path::{builder, Path};
use lyon::tessellation::{self, BuffersBuilder, FillOptions, FillTessellator, FillVertexConstructor, StrokeVertexConstructor, VertexBuffers};
use rusttype::{Font, IntoGlyphId, OutlineBuilder, Rect, Scale, ScaledGlyph};

pub mod text_shape;
pub use text_shape::TextShape;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertexs {
    pub position: [f32; 2],
    pub normal: [f32; 2],
    pub prim_id: u32,
}

#[derive(SystemParam)]
pub struct Config<'w,'s> {
   pub default_font: ResMut<'w,DefaultFont>,
   pub meshes: ResMut<'w,Assets<Mesh>>,
   pub materials: ResMut<'w,Assets<ColorMaterial>>,
   pub window: Query<'w,'s,&'static Window, With<PrimaryWindow>>,
}

#[derive(Resource)]
pub struct DefaultFont(rusttype::Font<'static>);

impl<'s> Default for DefaultFont {
    fn default() -> Self {
        let font_data = include_bytes!("./fonts/yahei.ttf");
        let font = rusttype::Font::try_from_bytes(font_data as &[u8]).unwrap();
        Self(font)
    }
}

impl DefaultFont {
    pub fn font(&self) -> &rusttype::Font {
        &self.0
    }
}

pub struct WithId(pub u32);

impl FillVertexConstructor<Vertexs> for WithId {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> Vertexs {
        Vertexs {
            position: vertex.position().to_array(),
            normal: [0.0, 0.0],
            prim_id: self.0,
        }
    }
}

impl StrokeVertexConstructor<Vertexs> for WithId {
    fn new_vertex(&mut self, vertex: tessellation::StrokeVertex) -> Vertexs {
        Vertexs {
            position: vertex.position_on_path().to_array(),
            normal: vertex.normal().to_array(),
            prim_id: self.0,
        }
    }
}

pub struct Glpyh<'font> {
    pub scaled: ScaledGlyph<'font>,
    pub bounding_box: Rect<f32>,
}

impl<'font> Glpyh<'font> {
    pub fn new(font: &'font Font, id: impl IntoGlyphId, size: f32) -> Self {
        let scaled = font.glyph(id).scaled(Scale::uniform(size));
        let bounding_box = scaled.exact_bounding_box().unwrap();
        Self {
            scaled,
            bounding_box,
        }
    }

    pub fn write_path(
        &self,
        x: f32,
        y: f32,
        d: &mut String,
        p: &mut builder::WithSvg<BuilderImpl>,
    ) {
        let mut builder = XBuilder::new(
            x - self.bounding_box.min.x,
            y - self.bounding_box.min.y,
            d,
            p,
        );
        self.scaled.build_outline(&mut builder);
    }

    pub fn path(&self, x: f32, y: f32) -> svg::node::element::Path {
        let mut d = String::new();
        let mut path = Path::builder().with_svg();
        self.write_path(x, y, &mut d, &mut path);
        svg::node::element::Path::new()
            .set("d", d)
            .set("fill", "#000")
    }
}

pub struct XBuilder<'a> {
    pub x: f32,
    pub y: f32,
    pub d: &'a mut String,
    pub path: &'a mut builder::WithSvg<BuilderImpl>,
}

impl<'a> XBuilder<'a> {
    pub fn new(
        x: f32,
        y: f32,
        d: &'a mut String,
        p: &'a mut builder::WithSvg<BuilderImpl>,
    ) -> Self {
        Self { x, y, d, path: p }
    }
}

impl OutlineBuilder for XBuilder<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.path.move_to(point(x + self.x, y + self.y));
        self.d.push_str(&format!("M{} {}", x + self.x, y + self.y));
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to(point(x + self.x, y + self.y));
        self.d.push_str(&format!("L{} {}", x + self.x, y + self.y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.path.quadratic_bezier_to(
            point(x1 + self.x, y1 + self.y),
            point(x + self.x, y + self.y),
        );
        self.d.push_str(&format!(
            "Q{} {},{} {}",
            x1 + self.x,
            y1 + self.y,
            x + self.x,
            y + self.y
        ));
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.path.cubic_bezier_to(
            point(x1 + self.x, y1 + self.y),
            point(x2 + self.x, y2 + self.y),
            point(x + self.x, y + self.y),
        );
        self.d.push_str(&format!(
            "C{} {},{} {},{} {}",
            x1 + self.x,
            y1 + self.y,
            x2 + self.x,
            y2 + self.y,
            x + self.x,
            y + self.y
        ));
    }

    fn close(&mut self) {
        self.path.close();
        self.d.push('Z');
    }
}

pub fn get_mesh_text(str:&String,height:&mut f32,length:&mut f32) -> Mesh {
    let font_data = include_bytes!("./fonts/yahei.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    let text = TextShape::builder()
        .size(1.0)
        .build(&font, str);
    *height = text.height;
    *length = text.witdh;
    let mut fill_tess = FillTessellator::new();

    let tolerance = 0.005;
    let fill_prim_id = 1;
    let mut geometry: VertexBuffers<Vertexs, u16> = VertexBuffers::new();

    fill_tess
        .tessellate_path(
            &text.xpath,
            &FillOptions::tolerance(tolerance).with_fill_rule(tessellation::FillRule::NonZero),
            &mut BuffersBuilder::new(&mut geometry, WithId(fill_prim_id as u32)),
        )
        .unwrap();

    let mut positions = Vec::new();
    let indices = bevy::render::mesh::Indices::U16(geometry.indices);
    for vertice in geometry.vertices {
        positions.push([vertice.position[0],0.,vertice.position[1]]);
    }

    let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList,RenderAssetUsages::default());
    mesh.insert_indices(indices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    mesh =  mesh.transformed_by(Transform::from_translation(Vec3::new(-text.witdh/2.,0.,-text.height/2.)));
    mesh
}



