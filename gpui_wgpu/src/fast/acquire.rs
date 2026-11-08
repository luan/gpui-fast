// Modified by Longbridge for gpui-fast.
//! Acquiring the surface texture before the frame is built.
//!
//! `draw` asks the surface for a texture once the scene is ready, and waits
//! there while the compositor holds every drawable. The frame it then
//! presents was built before the wait. A host that calls
//! `WgpuRenderer::acquire_surface_texture` before building the frame waits
//! first instead, so the presented frame shows the state after the wait.

use crate::WgpuRenderer;
use crate::wgpu_renderer::RendererState;

/// A surface texture acquired before `draw`, which presents into it.
#[derive(Default)]
pub(crate) struct AcquiredTexture(Option<wgpu::SurfaceTexture>);

impl WgpuRenderer {
    /// Acquires the next surface texture now rather than in `draw`, so that
    /// waiting for one happens before the frame is built. `draw` presents into
    /// the held texture. Returns whether a texture is held.
    pub fn acquire_surface_texture(&mut self) -> bool {
        if self.fast_acquired.0.is_none()
            && let RendererState::Ready { surface, .. } = &self.state
            && let wgpu::CurrentSurfaceTexture::Success(texture) = surface.get_current_texture()
        {
            self.fast_acquired.0 = Some(texture);
        }
        self.fast_acquired.0.is_some()
    }
}

/// The texture `draw` presents into: the one acquired ahead of it, or the
/// surface's next.
pub(crate) fn current_texture(
    acquired: &mut AcquiredTexture,
    surface: &wgpu::Surface,
) -> wgpu::CurrentSurfaceTexture {
    acquired.0.take().map_or_else(
        || surface.get_current_texture(),
        wgpu::CurrentSurfaceTexture::Success,
    )
}

/// Drops a held texture, which has to happen before its surface is
/// reconfigured, replaced or released.
pub(crate) fn release(acquired: &mut AcquiredTexture) {
    acquired.0 = None;
}
