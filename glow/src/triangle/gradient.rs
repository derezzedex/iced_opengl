use crate::program::Version;
use crate::triangle::{set_transform, simple_triangle_program};
use glow::{Context, HasContext, NativeProgram};
use iced_graphics::gradient::Gradient;
use iced_graphics::gradient::Linear;
use iced_graphics::Transformation;

#[derive(Debug)]
pub struct GradientProgram {
    pub program: <Context as HasContext>::Program,
    pub uniform_data: GradientUniformData,
}

#[derive(Debug)]
pub struct GradientUniformData {
    gradient: Gradient,
    transform: Transformation,
    uniform_locations: GradientUniformLocations,
}

#[derive(Debug)]
struct GradientUniformLocations {
    gradient_direction_location: <Context as HasContext>::UniformLocation,
    color_stops_size_location: <Context as HasContext>::UniformLocation,
    //currently the maximum number of stops is 64 due to needing to allocate the
    //memory for the array of stops with a const value in GLSL
    color_stops_location: <Context as HasContext>::UniformLocation,
    transform_location: <Context as HasContext>::UniformLocation,
}

impl GradientProgram {
    pub fn new(gl: &Context, shader_version: &Version) -> Self {
        let program = simple_triangle_program(
            gl,
            shader_version,
            include_str!("../shader/common/gradient.frag"),
        );

        Self {
            program,
            uniform_data: GradientUniformData::new(gl, program),
        }
    }

    pub fn write_uniforms(
        &mut self,
        gl: &Context,
        gradient: &Gradient,
        transform: &Transformation,
    ) {
        if transform != &self.uniform_data.transform {
            set_transform(
                gl,
                self.uniform_data.uniform_locations.transform_location,
                *transform,
            );
        }

        if &self.uniform_data.gradient != gradient {
            match gradient {
                Gradient::Linear(linear) => {
                    unsafe {
                        gl.uniform_4_f32(
                            Some(
                                &self.uniform_data.uniform_locations.gradient_direction_location
                            ),
                            linear.start.x,
                            linear.start.y,
                            linear.end.x,
                            linear.end.y
                        );

                        gl.uniform_1_u32(
                            Some(
                                &self
                                    .uniform_data
                                    .uniform_locations
                                    .color_stops_size_location,
                            ),
                            (linear.color_stops.len() * 2) as u32,
                        );

                        let mut stops = [0.0; 128];

                        for (index, stop) in linear.color_stops.iter().enumerate() {
                            if index == 16 { break; }
                            stops[index*8] = stop.color.r;
                            stops[(index*8)+1] = stop.color.g;
                            stops[(index*8)+2] = stop.color.b;
                            stops[(index*8)+3] = stop.color.a;
                            stops[(index*8)+4] = stop.offset;
                            stops[(index*8)+5] = 0.;
                            stops[(index*8)+6] = 0.;
                            stops[(index*8)+7] = 0.;
                        }

                        gl.uniform_4_f32_slice(
                            Some(
                                &self
                                    .uniform_data
                                    .uniform_locations
                                    .color_stops_location,
                            ),
                            &stops,
                        );
                    }
                }
            }

            self.uniform_data.gradient = gradient.clone();
        }
    }

    pub fn use_program(
        &mut self,
        gl: &Context,
        gradient: &Gradient,
        transform: &Transformation,
    ) {
        unsafe { gl.use_program(Some(self.program)) }

        self.write_uniforms(gl, gradient, transform);
    }
}

impl GradientUniformData {
    fn new(gl: &Context, program: NativeProgram) -> Self {
        let gradient_direction_location =
            unsafe { gl.get_uniform_location(program, "gradient_direction") }
                .expect("Gradient - Get gradient_direction.");

        let color_stops_size_location =
            unsafe { gl.get_uniform_location(program, "color_stops_size") }
                .expect("Gradient - Get color_stops_size.");

        let color_stops_location = unsafe {
            gl.get_uniform_location(program, "color_stops")
                .expect("Gradient - Get color_stops.")
        };

        let transform_location =
            unsafe { gl.get_uniform_location(program, "u_Transform") }
                .expect("Get transform location.");

        GradientUniformData {
            gradient: Gradient::Linear(Linear {
                start: Default::default(),
                end: Default::default(),
                color_stops: vec![],
            }),
            transform: Transformation::identity(),
            uniform_locations: GradientUniformLocations {
                gradient_direction_location,
                color_stops_size_location,
                color_stops_location,
                transform_location,
            },
        }
    }
}
