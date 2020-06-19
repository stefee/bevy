use crate::{Font, FontAtlasSet};
use bevy_asset::Assets;
use bevy_render::{
    draw::{Draw, DrawContext, DrawError, Drawable},
    mesh,
    pipeline::PipelineSpecialization,
    render_resource::{
        AssetRenderResourceBindings, BindGroup, BufferUsage, RenderResourceBindings,
        RenderResourceId,
    },
    Color,
};
use bevy_sprite::{TextureAtlas, TextureAtlasSprite};
use glam::Vec3;

pub struct TextStyle {
    pub font_size: f32,
    pub color: Color,
}

#[allow(dead_code)]
pub struct DrawableText<'a> {
    font: &'a Font,
    font_atlas_set: &'a FontAtlasSet,
    texture_atlases: &'a Assets<TextureAtlas>,
    render_resource_bindings: &'a mut RenderResourceBindings,
    asset_render_resource_bindings: &'a mut AssetRenderResourceBindings,
    position: Vec3,
    style: &'a TextStyle,
    text: &'a str,
}

impl<'a> DrawableText<'a> {
    pub fn new(
        font: &'a Font,
        font_atlas_set: &'a FontAtlasSet,
        texture_atlases: &'a Assets<TextureAtlas>,
        render_resource_bindings: &'a mut RenderResourceBindings,
        asset_render_resource_bindings: &'a mut AssetRenderResourceBindings,
        position: Vec3,
        style: &'a TextStyle,
        text: &'a str,
    ) -> Self {
        Self {
            font,
            font_atlas_set,
            texture_atlases,
            render_resource_bindings,
            asset_render_resource_bindings,
            position,
            style,
            text,
        }
    }
}

impl<'a> Drawable for DrawableText<'a> {
    fn draw(&mut self, draw: &mut Draw, context: &mut DrawContext) -> Result<(), DrawError> {
        context.set_pipeline(
            draw,
            bevy_sprite::SPRITE_SHEET_PIPELINE_HANDLE,
            PipelineSpecialization::empty(),
        )?;

        let render_resource_context = &**context.render_resource_context;
        if let Some(RenderResourceId::Buffer(quad_vertex_buffer)) = render_resource_context
            .get_asset_resource(bevy_sprite::QUAD_HANDLE, mesh::VERTEX_BUFFER_ASSET_INDEX)
        {
            draw.set_vertex_buffer(0, quad_vertex_buffer, 0);
        }
        let mut indices = 0..0;
        if let Some(RenderResourceId::Buffer(quad_index_buffer)) = render_resource_context
            .get_asset_resource(bevy_sprite::QUAD_HANDLE, mesh::INDEX_BUFFER_ASSET_INDEX)
        {
            draw.set_index_buffer(quad_index_buffer, 0);
            if let Some(buffer_info) = render_resource_context.get_buffer_info(quad_index_buffer) {
                indices = 0..(buffer_info.size / 2) as u32;
            } else {
                panic!("expected buffer type");
            }
        }

        let current_position = Vec3::new(0.0, 0.0, 0.0);
        // set global bindings
        context.set_bind_groups_from_bindings(draw, &mut [self.render_resource_bindings])?;

        // set local per-character bindings
        for character in self.text.chars() {
            if let Some(glyph_atlas_info) = self
                .font_atlas_set
                .get_glyph_atlas_info(self.style.font_size, character)
            {
                let atlas_render_resource_bindings = self
                    .asset_render_resource_bindings
                    .get_mut(glyph_atlas_info.texture_atlas)
                    .unwrap();
                context
                    .set_bind_groups_from_bindings(draw, &mut [atlas_render_resource_bindings])?;
                let sprite_buffer = TextureAtlasSprite {
                    index: glyph_atlas_info.char_index,
                    position: current_position,
                    scale: 1.0,
                };

                let sprite_buffer = context
                    .shared_buffers
                    .get_buffer(&sprite_buffer, BufferUsage::UNIFORM)
                    .unwrap();
                let sprite_bind_group = BindGroup::build().add_binding(0, sprite_buffer).finish();
                context.create_bind_group_resource(2, &sprite_bind_group)?;
                draw.set_bind_group(2, &sprite_bind_group);
                draw.draw_indexed(indices.clone(), 0, 0..1);
            }
        }
        Ok(())
    }
}