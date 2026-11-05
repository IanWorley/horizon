//! A still picture of a player wearing items, drawn on the CPU (the launcher's Wardrobe): the
//! player model at rest with its skin, each item on its bone, seen from the front at an angle. A
//! companion floats at its anchor, an aura sits at the feet see-through, and a trail shows as a
//! few of its particles left behind.

use crate::item::cube_faces;
use crate::{Face, Item, Part, Slot, Texture, pose_matrix};
use glam::{Mat4, Vec3, Vec4Swizzles};
use std::borrow::Cow;

/// RGBA pixels, row by row, straight alpha.
#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// The player model's parts at rest, as vanilla's `PlayerModel` lays them out on a 64×64 skin:
/// each body part with its outer layer (`hat`, `jacket`, sleeves, pants) as a child. `slim` is the
/// three-pixel-wide arms of a slim skin.
pub fn player_model(slim: bool) -> Part {
    let tex = [64.0, 64.0];
    let part = |name: &str, pivot: [f32; 3], origin, size, uv, outer: (&str, [f32; 2], f32)| Part {
        name: name.into(),
        pose: [pivot[0], pivot[1], pivot[2], 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        faces: cube_faces(origin, size, uv, 0.0, false, tex),
        children: vec![Part {
            name: outer.0.into(),
            pose: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            faces: cube_faces(origin, size, outer.1, outer.2, false, tex),
            children: Vec::new(),
        }],
    };
    let (arm, arm_y, right_x) = if slim {
        ([3.0, 12.0, 4.0], 2.5, -2.0)
    } else {
        ([4.0, 12.0, 4.0], 2.0, -3.0)
    };
    let leg = [4.0, 12.0, 4.0];
    Part {
        name: "root".into(),
        pose: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        faces: Vec::new(),
        children: vec![
            part(
                "head",
                [0.0; 3],
                [-4.0, -8.0, -4.0],
                [8.0; 3],
                [0.0, 0.0],
                ("hat", [32.0, 0.0], 0.5),
            ),
            part(
                "body",
                [0.0; 3],
                [-4.0, 0.0, -2.0],
                [8.0, 12.0, 4.0],
                [16.0, 16.0],
                ("jacket", [16.0, 32.0], 0.25),
            ),
            part(
                "right_arm",
                [-5.0, arm_y, 0.0],
                [right_x, -2.0, -2.0],
                arm,
                [40.0, 16.0],
                ("right_sleeve", [40.0, 32.0], 0.25),
            ),
            part(
                "left_arm",
                [5.0, arm_y, 0.0],
                [-1.0, -2.0, -2.0],
                arm,
                [32.0, 48.0],
                ("left_sleeve", [48.0, 48.0], 0.25),
            ),
            part(
                "right_leg",
                [-1.9, 12.0, 0.0],
                [-2.0, 0.0, -2.0],
                leg,
                [0.0, 16.0],
                ("right_pants", [0.0, 32.0], 0.25),
            ),
            part(
                "left_leg",
                [1.9, 12.0, 0.0],
                [-2.0, 0.0, -2.0],
                leg,
                [16.0, 48.0],
                ("left_pants", [0.0, 48.0], 0.25),
            ),
        ],
    }
}

/// The matrix of the part named `name` under `parent`, at rest.
pub fn rest_matrix(part: &Part, parent: Mat4, name: &str) -> Option<Mat4> {
    let m = parent * pose_matrix(part.pose);
    if part.name == name {
        return Some(m);
    }
    part.children.iter().find_map(|c| rest_matrix(c, m, name))
}

/// One model to draw: its root under `base` (model space, blocks), its parts posed by
/// `poses` (name → pose; the rest at rest), at `alpha` (1: solid, cut out as entities are).
pub struct Draw<'a> {
    pub root: Cow<'a, Part>,
    pub base: Mat4,
    pub texture: &'a Texture,
    pub poses: Vec<(String, crate::Pose)>,
    pub alpha: f32,
}

impl<'a> Draw<'a> {
    /// `root` at rest, solid, where it is: the player model with its skin, say.
    pub fn model(root: &'a Part, texture: &'a Texture) -> Self {
        Self {
            root: Cow::Borrowed(root),
            base: Mat4::IDENTITY,
            texture,
            poses: Vec::new(),
            alpha: 1.0,
        }
    }

    /// `item` on the player model `body`, at `ticks` of its animations: one draw per bone it
    /// rides (a shirt's sleeves are on the arms); a companion at its anchor, an aura at the feet,
    /// a trail as particles left behind.
    pub fn item(item: &'a Item, body: &Part, ticks: f32) -> Vec<Self> {
        let mut poses = Vec::new();
        item.animate(ticks, |name, pose| poses.push((name.to_owned(), pose)));
        let at = |base: Mat4, alpha: f32| Self {
            root: Cow::Borrowed(&item.root),
            base,
            texture: &item.texture,
            poses: poses.clone(),
            alpha,
        };
        match item.slot {
            Slot::Companion => {
                let anchor = item.companion.as_ref().map_or([0.0; 3], |c| c.anchor);
                return vec![at(Mat4::from_translation(Vec3::from(anchor) / 16.0), 1.0)];
            }
            Slot::Aura => {
                let alpha = item.aura.as_ref().map_or(1.0, |a| a.alpha);
                // The middle of the feet: 24 pixels under the neck.
                return vec![at(Mat4::from_translation(Vec3::Y * 1.5), alpha)];
            }
            Slot::Trail => {
                return trail_part(item)
                    .map(|root| Self {
                        root: Cow::Owned(root),
                        base: Mat4::IDENTITY,
                        texture: &item.texture,
                        poses: Vec::new(),
                        alpha: 1.0,
                    })
                    .into_iter()
                    .collect();
            }
            _ => {}
        }
        item.roots()
            .filter_map(|(bone, root)| {
                Some(Self {
                    root: Cow::Borrowed(root),
                    base: rest_matrix(body, Mat4::IDENTITY, bone.part())?,
                    texture: &item.texture,
                    poses: poses.clone(),
                    alpha: 1.0,
                })
            })
            .collect()
    }
}

/// A trail as a still shows it: fourteen of its particles left behind a player walking forward, as
/// cards facing the front, each older one farther back, fallen (or risen) as far as its age
/// says and smaller as it fades.
pub fn trail_part(item: &Item) -> Option<Part> {
    let t = item.trail.as_ref()?;
    let n = 14;
    let hash = |k: u32| ((k.wrapping_mul(2_654_435_761) >> 8) & 0xFFFF) as f32 / 65535.0;
    let spacing = (16.0 / t.rate).min(2.2);
    let ground = match t.from {
        crate::From::Feet => 22.4,
        crate::From::Body => 9.6,
    };
    let faces = (0..n)
        .map(|k| {
            let life = t.lifetime as f32;
            let age = (k as f32 + 0.5) / n as f32 * life;
            let side = if k % 2 == 0 { 1.0 } else { -1.0 };
            let x = side * (2.0 + 3.0 * hash(k)) + t.drift * (age * 0.2 + k as f32).sin();
            let y = (ground + 0.5 * t.gravity * age * age * 16.0).min(24.0);
            let z = 3.0 + k as f32 * spacing;
            let mut size = t.size[0] + (t.size[1] - t.size[0]) * hash(k + 7);
            if t.fade {
                size *= (3.0 * (1.0 - age / life)).clamp(0.2, 1.0);
            }
            let h = size / 2.0;
            let sprite = (k % t.sprites) as f32;
            let (u0, u1) = (sprite / t.sprites as f32, (sprite + 1.0) / t.sprites as f32);
            Face {
                normal: [0.0, 0.0, -1.0],
                vertices: [
                    [x + h, y - h, z, u1, 0.0],
                    [x - h, y - h, z, u0, 0.0],
                    [x - h, y + h, z, u0, 1.0],
                    [x + h, y + h, z, u1, 1.0],
                ],
            }
        })
        .collect();
    Some(Part {
        name: "trail".into(),
        pose: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        faces,
        children: Vec::new(),
    })
}

/// Draws `draws` into a `width`×`height` picture: turned `yaw` degrees (0 shows the front),
/// tilted `pitch` degrees down: 2.5 blocks to the height, the head's top a tenth of the way down
/// (room for a hat).
pub fn render(draws: &[Draw], width: u32, height: u32, yaw: f32, pitch: f32) -> Image {
    render_zoomed(draws, width, height, yaw, pitch, 1.0)
}

/// [`render`] with the body `zoom` times as tall (below 1: room around it for a companion, a
/// trail or an aura), its middle kept in the middle of the picture.
pub fn render_zoomed(
    draws: &[Draw],
    width: u32,
    height: u32,
    yaw: f32,
    pitch: f32,
    zoom: f32,
) -> Image {
    let (w, h) = (width as usize, height as usize);
    let mut rgba = vec![0u8; w * h * 4];
    let mut depth = vec![f32::INFINITY; w * h];
    // Model space has y down and the front at -z: the camera looks along +z from the front. The
    // body is 2 blocks tall, 1 block from the head's top to its middle.
    let scale = height as f32 / 2.5 * zoom;
    let view = Mat4::from_translation(Vec3::new(width as f32 / 2.0, height as f32 / 2.0 - scale, 0.0))
        * Mat4::from_scale(Vec3::new(scale, scale, scale))
        * Mat4::from_rotation_x(-pitch.to_radians())
        * Mat4::from_rotation_y(yaw.to_radians())
        // Up the body's pivot: the head's top is 0.5 blocks above it.
        * Mat4::from_translation(Vec3::new(0.0, 0.5, 0.0));
    // Solid first; see-through over them, unsorted (an aura is one thin layer).
    let solid = draws.iter().filter(|d| d.alpha >= 1.0);
    for d in solid.chain(draws.iter().filter(|d| d.alpha < 1.0)) {
        let mut stack = vec![(&*d.root, d.base)];
        while let Some((part, parent)) = stack.pop() {
            let pose = d
                .poses
                .iter()
                .find(|(n, _)| *n == part.name)
                .map_or(part.pose, |(_, p)| *p);
            let m = parent * pose_matrix(pose);
            let to_screen = view * m;
            let normal = glam::Mat3::from_mat4(view * m);
            for face in &part.faces {
                let n = (normal * Vec3::from_array(face.normal)).normalize_or_zero();
                // Seen through, a face turned away would show behind the near one: skip it.
                if d.alpha < 1.0 && n.z > 0.0 {
                    continue;
                }
                // Faces turned away are hidden by the depth test; the light is vanilla-like:
                // brighter facing up and toward the viewer.
                let light = (0.55 + 0.25 * (-n.y).max(0.0) + 0.2 * (-n.z).max(0.0)).min(1.0);
                let v: Vec<(Vec3, [f32; 2])> = face
                    .vertices
                    .iter()
                    .map(|v| {
                        let p =
                            to_screen * glam::Vec4::new(v[0] / 16.0, v[1] / 16.0, v[2] / 16.0, 1.0);
                        (p.xyz(), [v[3], v[4]])
                    })
                    .collect();
                for tri in [[0, 1, 2], [0, 2, 3]] {
                    raster(
                        [v[tri[0]], v[tri[1]], v[tri[2]]],
                        d.texture,
                        (light, d.alpha),
                        (w, h),
                        &mut rgba,
                        &mut depth,
                    );
                }
            }
            for child in &part.children {
                stack.push((child, m));
            }
        }
    }
    Image {
        width,
        height,
        rgba,
    }
}

fn raster(
    v: [(Vec3, [f32; 2]); 3],
    tex: &Texture,
    (light, alpha): (f32, f32),
    (w, h): (usize, usize),
    rgba: &mut [u8],
    depth: &mut [f32],
) {
    let (a, b, c) = (v[0].0, v[1].0, v[2].0);
    let area = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if area.abs() < 1e-6 {
        return;
    }
    let x0 = a.x.min(b.x).min(c.x).floor().max(0.0) as usize;
    let x1 = (a.x.max(b.x).max(c.x).ceil() as usize).min(w);
    let y0 = a.y.min(b.y).min(c.y).floor().max(0.0) as usize;
    let y1 = (a.y.max(b.y).max(c.y).ceil() as usize).min(h);
    for y in y0..y1 {
        for x in x0..x1 {
            let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
            let w0 = ((b.x - px) * (c.y - py) - (b.y - py) * (c.x - px)) / area;
            let w1 = ((c.x - px) * (a.y - py) - (c.y - py) * (a.x - px)) / area;
            let w2 = 1.0 - w0 - w1;
            if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                continue;
            }
            let z = w0 * a.z + w1 * b.z + w2 * c.z;
            let i = y * w + x;
            if z >= depth[i] {
                continue;
            }
            let u = w0 * v[0].1[0] + w1 * v[1].1[0] + w2 * v[2].1[0];
            let t = w0 * v[0].1[1] + w1 * v[1].1[1] + w2 * v[2].1[1];
            let tx = ((u * tex.width as f32) as usize).min(tex.width as usize - 1);
            let ty = ((t * tex.height as f32) as usize).min(tex.height as usize - 1);
            let texel = &tex.rgba[(ty * tex.width as usize + tx) * 4..][..4];
            // Cutout, as entity textures draw.
            if texel[3] < 128 {
                continue;
            }
            if alpha < 1.0 {
                // See-through: over what is there, without hiding what comes behind.
                let a = alpha * f32::from(texel[3]) / 255.0;
                let under = f32::from(rgba[i * 4 + 3]) / 255.0;
                let out = a + under * (1.0 - a);
                for k in 0..3 {
                    let over = f32::from(texel[k]) * light;
                    let was = f32::from(rgba[i * 4 + k]) * under;
                    rgba[i * 4 + k] = ((over * a + was * (1.0 - a)) / out.max(1e-6)).round() as u8;
                }
                rgba[i * 4 + 3] = (out * 255.0).round() as u8;
                continue;
            }
            depth[i] = z;
            for k in 0..3 {
                rgba[i * 4 + k] = (f32::from(texel[k]) * light).round() as u8;
            }
            rgba[i * 4 + 3] = 255;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 64×64 skin in one colour per region: what lands where shows which face was drawn.
    fn skin() -> Texture {
        let mut rgba = vec![0u8; 64 * 64 * 4];
        for y in 0..64 {
            for x in 0..64 {
                let c: [u8; 4] = match (x, y) {
                    // The head's front (box UV: u+d, v+d, 8×8 from (0, 0)).
                    (8..16, 8..16) => [200, 0, 0, 255],
                    (0..32, 0..16) => [0, 200, 0, 255],
                    // The hat layer: transparent.
                    (32..64, 0..16) => [0, 0, 0, 0],
                    _ => [0, 0, 200, 255],
                };
                rgba[(y * 64 + x) * 4..][..4].copy_from_slice(&c);
            }
        }
        Texture {
            width: 64,
            height: 64,
            rgba,
        }
    }

    fn at(img: &Image, x: u32, y: u32) -> [u8; 4] {
        img.rgba[((y * img.width + x) * 4) as usize..][..4]
            .try_into()
            .unwrap()
    }

    #[test]
    fn the_front_of_the_head_faces_the_viewer() {
        let body = player_model(false);
        let tex = skin();
        let draw = Draw::model(&body, &tex);
        let img = render(&[draw], 100, 100, 0.0, 0.0);
        // 40 px a block: the head's top at row 10, its bottom (the pivot) 20 px lower; its
        // middle at row 20, column 50.
        let face = at(&img, 50, 20);
        assert_eq!(face[3], 255);
        assert!(face[0] > 100 && face[1] == 0 && face[2] == 0, "{face:?}");
        // Above the head, nothing (the transparent hat layer doesn't hide it).
        assert_eq!(at(&img, 50, 5)[3], 0);
        // Turned round: the back of the head (green region), not the face.
        let back = render(&[Draw::model(&body, &tex)], 100, 100, 180.0, 0.0);
        let c = at(&back, 50, 20);
        assert!(c[1] > 100 && c[0] == 0, "{c:?}");
    }

    #[test]
    fn an_item_sits_on_its_bone() {
        let file = format!(
            r#"{{"format": 1, "id": "cap", "slot": "head", "bone": "head",
              "attach": {{"offset": [0, -8, 0]}}, "texture": {{"png": "{}"}},
              "parts": [{{"name": "c", "cubes": [{{"origin": [-4, -4, -4], "size": [8, 4, 8], "uv": [0, 0]}}]}}]}}"#,
            crate::pose::tests::png(32, 32)
        );
        let item = Item::parse(file.as_bytes()).unwrap();
        let body = player_model(false);
        let tex = skin();
        let mut draws = vec![Draw::model(&body, &tex)];
        let bare = render(&draws, 100, 100, 0.0, 0.0);
        draws.extend(Draw::item(&item, &body, 0.0));
        let worn = render(&draws, 100, 100, 0.0, 0.0);
        // The cap spans 8 to 12 model pixels above the head's pivot: rows 0 to 10, over the
        // head's top at row 10.
        assert_eq!(at(&bare, 50, 5)[3], 0);
        let cap = at(&worn, 50, 5);
        assert_eq!(cap[3], 255);
        // Grey (200 lit), not the skin's colours.
        assert!(cap[0] == cap[1] && cap[1] == cap[2], "{cap:?}");
        // The face is still the face.
        assert_eq!(at(&worn, 50, 20), at(&bare, 50, 20));
    }

    fn worn(file: &str, x: u32, y: u32) -> ([u8; 4], [u8; 4]) {
        let item = Item::parse(file.as_bytes()).unwrap();
        let body = player_model(false);
        let tex = skin();
        let mut draws = vec![Draw::model(&body, &tex)];
        let bare = render(&draws, 100, 100, 0.0, 0.0);
        draws.extend(Draw::item(&item, &body, 0.0));
        let worn = render(&draws, 100, 100, 0.0, 0.0);
        (at(&bare, x, y), at(&worn, x, y))
    }

    #[test]
    fn a_companion_floats_at_its_anchor_beside_the_head() {
        // A 4-pixel cube 14 pixels to the player's right of the neck, 6 up: 40 px a block puts
        // its middle at column 50 - 35, row 10 + (0.5 - 0.375) * 40.
        let file = format!(
            r#"{{"format": 1, "id": "pal", "slot": "companion", "texture": {{"png": "{}"}},
              "parts": [{{"name": "c", "cubes": [{{"origin": [-2, -2, -2], "size": [4, 4, 4], "uv": [0, 0]}}]}}],
              "companion": {{"anchor": [-14, -6, 8]}}}}"#,
            crate::pose::tests::png(16, 16)
        );
        let (bare, worn) = worn(&file, 15, 15);
        assert_eq!(bare[3], 0);
        assert_eq!(worn[3], 255);
    }

    #[test]
    fn an_aura_at_the_feet_lets_the_legs_show_through() {
        // A wall 24 pixels wide and 6 high, 12 in front of the feet, at half opacity: rows 75
        // to 90 of the picture, columns 20 to 80.
        let file = format!(
            r#"{{"format": 1, "id": "glow", "slot": "aura", "texture": {{"png": "{}"}},
              "parts": [{{"name": "c", "cubes": [{{"origin": [-12, -6, -12], "size": [24, 6, 1], "uv": [0, 0]}}]}}],
              "aura": {{"at": "feet", "alpha": 0.5}}}}"#,
            crate::pose::tests::png(64, 16)
        );
        // The test texture is grey 200 at 200 of 255 opaque, lit 0.75 facing the viewer: 150,
        // covering 0.5 * 200 / 255 of what is behind.
        let a = 0.5 * 200.0 / 255.0;
        let (bare, aura) = worn(&file, 25, 82);
        assert_eq!(bare[3], 0);
        assert_eq!(aura, [150, 150, 150, (a * 255.0_f32).round() as u8]);
        // Over a leg: the leg's blue (150 lit) still shows through the grey.
        let (leg, over) = worn(&file, 45, 82);
        assert_eq!(leg, [0, 0, 150, 255]);
        assert_eq!(
            over,
            [
                (150.0 * a).round() as u8,
                (150.0 * a).round() as u8,
                150,
                255
            ]
        );
    }

    #[test]
    fn a_trail_leaves_particles_behind_on_the_ground() {
        let file = format!(
            r#"{{"format": 1, "id": "leaves", "slot": "trail", "texture": {{"png": "{}"}}, "parts": [],
              "trail": {{"sprites": 4, "rate": 2, "lifetime": 30, "size": [2, 3], "gravity": 0.01, "fade": true}}}}"#,
            crate::pose::tests::png(32, 8)
        );
        let item = Item::parse(file.as_bytes()).unwrap();
        let part = trail_part(&item).unwrap();
        assert_eq!(part.faces.len(), 14);
        // Each farther behind than the last (+z), none under the ground (24 pixels down).
        let z: Vec<f32> = part.faces.iter().map(|f| f.vertices[0][2]).collect();
        assert!(z.windows(2).all(|w| w[1] > w[0]), "{z:?}");
        assert!(
            part.faces
                .iter()
                .flat_map(|f| f.vertices)
                .all(|v| v[1] <= 24.0 + 1.5)
        );
    }
}
