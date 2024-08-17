use bevy::{math::{Vec2, Vec3}, prelude::{Mesh, Transform}, render::render_asset::RenderAssetUsages};
use lyon::{path::FillRule, tessellation::{BuffersBuilder, FillOptions, FillTessellator, StrokeTessellator, VertexBuffers}};
use rusttype::{Font, Point, Rect, Scale};
use svg::node::element::Path;

use super::WithId;

pub struct TextShape {
    pub path: Path,
    pub xpath: lyon::path::Path,
    pub witdh: f32,
    pub height: f32,
    pub bounding_box: Rect<f32>,
}

impl TextShape {
    pub fn new(path: Path, bounding_box: Rect<f32>, xpath: lyon::path::Path ) -> Self {
        Self { path, bounding_box, xpath ,witdh: 0.,height:0.}
    }

    pub fn builder() -> Builder<'static> {
        Default::default()
    }

    pub fn mesh(&self) -> Vec<Vec<Vec2>> {
        let mut tess = FillTessellator::new();

        let tolerance = 0.005;
        let fill_prim_id = 1;
        let mut geometry: VertexBuffers<super::Vertexs, u16> = VertexBuffers::new();
    
        tess
            .tessellate_path(
                &self.xpath,
                &FillOptions::tolerance(tolerance).with_fill_rule(FillRule::NonZero),
                &mut BuffersBuilder::new(&mut geometry, WithId(fill_prim_id as u32)),
            )
        .unwrap();
        
        let mut positions = Vec::new();
        let indices = bevy::render::mesh::Indices::U16(geometry.indices);
        let mut index_1 = 0.;
        let face =&mut Vec::new();
        for index in indices.iter() {
            index_1 += 1.;
            face.push(Vec2::new(geometry.vertices[index].position[0] ,-geometry.vertices[index].position[1]));
            if index_1 == 3. {
                positions.push(face.clone());
                face.clear();
                index_1 = 0.;
            }
        }
        positions
    }

    pub fn bevy_mesh(&self) -> Mesh{
        let mut tess = FillTessellator::new();

        let tolerance = 0.005;
        let fill_prim_id = 1;
        let mut geometry: VertexBuffers<super::Vertexs, u16> = VertexBuffers::new();
    
        tess
            .tessellate_path(
                &self.xpath,
                &FillOptions::tolerance(tolerance).with_fill_rule(FillRule::NonZero),
                &mut BuffersBuilder::new(&mut geometry, WithId(fill_prim_id as u32)),
            )
        .unwrap();
        
        let mut positions = Vec::new();
        let indices = bevy::render::mesh::Indices::U16(geometry.indices);
        for vertice in geometry.vertices {
            positions.push([vertice.position[0],-vertice.position[1],0.]);
        }

        let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList,RenderAssetUsages::default());
        mesh.insert_indices(indices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.duplicate_vertices();
        mesh.compute_flat_normals();
        mesh
    }
}

pub struct Builder<'a> {
    pub fill: &'a str,
    pub size: f32,
    pub start: Point<f32>,
    pub letter_spacing: f32,
}

impl Default for Builder<'static> {
    fn default() -> Self {
        Self {
            fill: "#000",
            size: 32.,
            start: Point { x: 0., y: 0. },
            letter_spacing: 1.,
        }
    }
}

impl Builder<'_> {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn start(mut self, start: Point<f32>) -> Self {
        self.start = start;
        self
    }

    pub fn build(&self, font: &Font, text: &str) -> TextShape {
        let mut d = String::new();
        let scale = Scale::uniform(self.size);
        let v_metrics = font.v_metrics(scale);
        
        let offset = Point {
            x: self.start.x,
            y: self.start.y + v_metrics.ascent,
        };

        let glyphs_height = v_metrics.ascent - v_metrics.descent;
        let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0);

        let mut p = lyon::path::Path::builder().with_svg();
        for glyph in glyphs {
            let glyph_unpositioned =  glyph.clone().into_unpositioned();
            glyph_unpositioned.build_outline(&mut super::XBuilder {
                x: glyph.position().x,
                y: glyph.position().y,
                d: &mut d,
                path: &mut p,
            });
        }

        let bounding_box = Rect {
            min: self.start,
            max: Point {
                x: width as f32,
                y: glyphs_height,
            },
        };

        let mut text = TextShape::new(Path::new().set("d", d).set("fill", "#000"), bounding_box, p.build());
        text.witdh  = width; 
        text.height = glyphs_height;
        text
    }
}


