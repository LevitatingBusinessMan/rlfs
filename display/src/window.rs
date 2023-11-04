use std::{rc::Rc, cell::RefCell, sync::{Mutex, Arc, RwLock}};
use slab_tree::{Tree, NodeId, NodeMut};

use crate::{FrameBuffer, Pixel, draw, BitMap};

type WindowId = NodeId;

pub struct Window {
	pub x: u32,
	pub y: u32,
	pub bitmap: BitMap,
}

impl Window {
	/// Init a (black) window
	pub fn create(width: u32, height: u32, x: u32, y: u32) -> Self {
		Self {
			x,
			y,
			bitmap: BitMap {
				height,
				width,
				pxs: vec![Pixel::new(0x00, 0x00, 0x00); (height * width) as usize].into_boxed_slice()
			},
		}
	}
	pub fn map(&self, fb: &mut BitMap) {
		draw::map(&self.bitmap, fb, self.x, self.y);
	}
	pub fn decorate(&self, fb: &mut FrameBuffer, v_info: &fb::var_screeninfo) {
		draw::draw_rect_border(self.bitmap.height, self.bitmap.height, self.x, self.y, Pixel::new(0xff, 0x00, 0x00), 1, fb, v_info);
	}
}

// pub fn draw(&self, fb: &mut FrameBuffer, v_info: &fb::var_screeninfo) {
// 	draw::draw_rect_border(self.width, self.height, self.x, self.y, Pixel::new(0x00, 0xff, 0x00, 0x00), 1, fb, v_info);
// 	draw::draw_rect(self.width, self.height, self.x, self.y, Pixel::new(0x00, 0x00, 0x00, 0x00), fb, v_info);
// }

pub(crate) fn create_root(v_info: &fb::var_screeninfo) -> Window {
	Window {
    x: 0,
    y: 0,
    bitmap:
		BitMap {
			width: v_info.xres,
			height: v_info.yres,
			pxs: vec![Pixel::new(0xff, 0xff, 0xff); (v_info.xres * v_info.yres) as usize].into_boxed_slice(),
		}
	}
}
