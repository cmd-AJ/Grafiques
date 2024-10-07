use crate::colors::Color;
use crate::material::Material;
use crate::texture::Texture;
use crate::Arc;
use crate::Cube;
use crate::Lazy;
use crate::Vec3;

static COBBLESTONE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/acacia.png")));
static SNOW: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/snowb.png")));
static SNOWBLOCK: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/coponieve.png")));
static GLASS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/glass.png")));
static ACACIADOWN: Lazy<Arc<Texture>> =
    Lazy::new(|| Arc::new(Texture::new("assets/acaciadown.png")));
static LEAVES: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/leaves.png")));
static RESPAWN: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/respawn.png")));

//transparency es (3)
//reflection (2)
//albedo (1)
//refraction 4

pub fn loadobjects(water: Material) -> Vec<Cube> {
    let marmle = Material::new(Color::new(118, 169, 253), 0.0, [0.5, 0.01, 0.8, 0.0], 0.0);

    let ivorys = Material::new_with_text(0.0, [1.0, 0.0, 0.0, 0.0], 0.0, COBBLESTONE.clone());

    let snowgras = Material::new_with_text(0.0, [0.9, 0.0, 0.0, 0.0], 0.0, SNOW.clone());

    let snowblocks = Material::new_with_text(0.0, [0.9, 0.0, 0.0, 0.0], 0.0, SNOWBLOCK.clone());

    let glassobj = Material::new_with_text(0.0, [1.0, 0.1, 0.2, 0.0], 0.0, GLASS.clone());

    let acacia_downs = Material::new_with_text(0.0, [1.0, 0.0, 0.0, 0.0], 0.0, ACACIADOWN.clone());
    let leaves = Material::new_with_text(0.0, [1.0, 0.0, 0.0, 0.0], 0.0, LEAVES.clone());

    let resp = Material::new_with_text(0.0, [1.0, 0.0, 0.0, 0.0], 0.0, RESPAWN.clone());

    let mut objects = Vec::new();
    let cloned_ivorys = ivorys.clone();
    let clone_acacia = acacia_downs.clone();
    let leaves = leaves.clone();

    let grid_size_x = 8; // Fixed size for the x dimension
    let grid_size_z = 6; // Fixed size for the z dimension
    let spacing = 0.5; // Adjust this to change the distance between cubes

    // You can adjust grid_size_y if needed
    let grid_size_y = 1; // Or keep it the same for y if desired

    for row in 0..grid_size_y {
        for col in 0..grid_size_x {
            for depth in 0..grid_size_z {
                objects.push(Cube {
                    center: Vec3::new(
                        col as f32 * spacing - 1.0,
                        row as f32 * spacing,
                        -depth as f32 * spacing + 0.5,
                    ),
                    size: 0.5,
                    material: snowgras.clone(),
                });
            }
        }
    }

    objects.push(Cube {
        center: Vec3::new(2.5 as f32, 0.5 as f32, -1 as f32),
        size: 0.5,
        material: snowblocks.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(
            2.5,  // x position remains constant
            1.0,  // y position remains constant
            -1.5, // z position increments by 0.5
        ),
        size: 0.5,                    // Size remains constant
        material: snowblocks.clone(), // Clone the material
    });

    objects.push(Cube {
        center: Vec3::new(
            2.5,  // x position remains constant
            0.5,  // y position remains constant
            -1.0, // z position increments by 0.5
        ),
        size: 0.5,                    // Size remains constant
        material: snowblocks.clone(), // Clone the material
    });

    objects.push(Cube {
        center: Vec3::new(2.5 as f32, 1.5 as f32, -1.0 as f32),
        size: 0.5,
        material: snowblocks.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(2.5 as f32, 0.5 as f32, -1.5 as f32),
        size: 0.5,
        material: snowblocks.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(
            2.5, // x position remains constant
            1.0, // y position remains constant
            -1.0,
        ),
        size: 0.5,
        material: glassobj.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(
            2.5, // x position remains constant
            1.0, // y position remains constant
            -0.5,
        ),
        size: 0.5,
        material: snowblocks.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(2.5 as f32, 0.5 as f32, -0.5 as f32),
        size: 0.5,
        material: snowblocks.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(2.0 as f32, 0.5 as f32, -0.0 as f32),
        size: 0.5,
        material: snowblocks.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(2.0 as f32, 0.5 as f32, -2.0 as f32),
        size: 0.5,
        material: snowblocks.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(1.5 as f32, 0.5 as f32, -2.0 as f32),
        size: 0.5,
        material: snowblocks.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(1.5 as f32, 0.5 as f32, -0.0 as f32),
        size: 0.5,
        material: snowblocks.clone(),
    });

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                1.0 - offset, // Increment x position
                0.5,          // Increment y position
                -0.5,         // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                1.0 - offset, // Increment x position
                0.5,          // Increment y position
                -1.5,         // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                2.0 - offset, // Increment x position
                1.5,          // Increment y position
                -1.5,         // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                2.0 - offset, // Increment x position
                1.0,          // Increment y position
                -2.0,         // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                2.0 - offset, // Increment x position
                1.0,          // Increment y position
                0.0,          // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                2.0 - offset, // Increment x position
                1.5,          // Increment y position
                -1.0,         // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                2.0 - offset, // Increment x position
                2.0,          // Increment y position
                -1.0,         // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                2.0 - offset, // Increment x position
                1.5,          // Increment y position
                -0.5,         // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                1.0 - offset, // Increment x position
                1.0,          // Increment y position
                -0.5,         // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                1.0 - offset, // Increment x position
                1.0,          // Increment y position
                -1.5,         // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                1.0 - offset, // Increment x position
                1.5,          // Increment y position
                -1.0,         // z remains constant
            ),
            size: 0.5,                    // Size remains constant
            material: snowblocks.clone(), // Clone the material
        });
    }

    objects.push(Cube {
        center: Vec3::new(0.5 as f32, 0.5 as f32, 0.5 as f32),
        size: 0.5,
        material: cloned_ivorys,
    });

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                -1.0 + offset, // Increment x position
                0.5,           // Increment y position
                0.0,           // z remains constant
            ),
            size: 0.5,                      // Size remains constant
            material: clone_acacia.clone(), // Clone the material
        });
    }

    let grid_size_x = 2; // Fixed size for the x dimension
    let grid_size_z = 6; // Fixed size for the z dimension
    let spacing = 0.5; // Adjust this to change the distance between cubes

    for row in 0..grid_size_y {
        for col in 0..grid_size_x {
            for depth in 0..grid_size_z {
                objects.push(Cube {
                    center: Vec3::new(
                        col as f32 * spacing - 2.5,
                        row as f32 * spacing,
                        -depth as f32 * spacing + 0.5,
                    ),
                    size: 0.5,
                    material: marmle.clone(),
                });
            }
        }
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                -1.0 - offset, // Increment x position
                0.5,           // Increment y position
                0.5,           // z remains constant
            ),
            size: 0.5,                // Size remains constant
            material: leaves.clone(), // Clone the material
        });
    }

    for g in 0..2 {
        let offset = g as f32 * 0.5; // Increment position by 1.0 for each cube
        objects.push(Cube {
            center: Vec3::new(
                -1.0 - offset, // Increment x position
                0.5,           // Increment y position
                -0.5,          // z remains constant
            ),
            size: 0.5,                // Size remains constant
            material: leaves.clone(), // Clone the material
        });
    }

    objects.push(Cube {
        center: Vec3::new(-1.5 as f32, 0.5 as f32, 0.0 as f32),
        size: 0.5,
        material: leaves,
    });

    objects.push(Cube {
        center: Vec3::new(-1.5, 0.0, 0.5),
        size: 0.5,
        material: marmle.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(-1.5, 0.0, -2.0),
        size: 0.5,
        material: marmle.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(-1.5, 0.0, -1.5),
        size: 0.5,
        material: water.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(-1.5, 0.0, -1.0),
        size: 0.5,
        material: water.clone(),
    });

    objects.push(Cube {
        center: Vec3::new(2.0, 0.5, -1.5),
        size: 0.5,
        material: resp.clone(),
    });

    objects
}
