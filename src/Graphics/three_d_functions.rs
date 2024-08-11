extern crate piston_window;
use crate::two_d_functions::*;
use piston_window::*;
use std::sync::{Arc, Mutex};
use piston_window::types::Triangle;
use std::sync::MutexGuard;
use nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct point3d
{
   pub x:f64,
   pub y:f64,
   pub z:f64,
}

impl point3d {
    fn sub(self, other: &point3d) -> Vector3<f64> {
      Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
  }







struct triangle_points3d<'a>
{
    triangle_point1: &'a point3d,
    triangle_point2: &'a point3d,
    triangle_point3: &'a point3d,
    color: &'a [f32; 4],
    lighting_on: &'a bool,
}



#[derive(Clone)]
pub struct triangle_point_index
{
   pub t1:usize,
   pub t2:usize,
   pub t3:usize,
}

pub struct object_3d
{
   pub points_3d:Vec<point3d>,
   pub triangles:Vec<triangle_point_index>,

}

pub struct surface
{
   pub points_3d:Vec<point3d>,
   pub lines:Vec<line_index>,

}
pub struct line_index
{
   pub line1:usize,
   pub line2:usize,

}
pub struct cad_object_3d
{
    pub name:String,
    pub points_3d:Vec<point3d>,
    pub triangles:Vec<triangle_point_index>,
    pub surfaces:Vec<surface>,
    pub color: [f32; 4],
    pub lighting_on: bool,

}


pub fn Begin_3d()
{

}



static camera_position:point3d = point3d { x: 1.0, y: 0.0, z: 0.0 };
pub fn render_3d(c:&Context, g: &mut G2d,global_vec:&Arc<MutexGuard<'_, Vec<cad_object_3d>>>,mut camera_rotation_x:f64,mut camera_rotation_y:f64,window_hight:f64, window_width:f64 ) {

    
    let mut triangles3d:Vec<triangle_points3d> = Vec::new();


    // Iterate over the global vector
    for obj in global_vec.iter() {
        for triangle in &obj.triangles
        {
           
            triangles3d.push(triangle_points3d{
                triangle_point1: &obj.points_3d[triangle.t1],
                triangle_point2: &obj.points_3d[triangle.t2],
                triangle_point3: &obj.points_3d[triangle.t3],
                color: &obj.color,
                lighting_on: &obj.lighting_on});
        }
    }
    


// ordertriangles
triangles3d.sort_by(|a, b| {
    let distance_a = distance_to_point(&camera_position, a);
    let distance_b = distance_to_point(&camera_position, b);
    distance_a.partial_cmp(&distance_b).unwrap()
  });
  let projection: Vec<Vec<f64>> = vec![
        vec![1.0, 0.0, 0.0],  // First row
        vec![0.0, 1.0, 0.0],  // Second row
      ];


    //rotate
      //float[,] rotationy = { { MathF.Cos(cameray), 0, -MathF.Sin(cameray) }, { 0, 1, 0 }, { MathF.Sin(cameray), 0, MathF.Cos(cameray) } };
      //float[,] rotationx = { { 1, 0, 0 }, { 0, MathF.Cos(camerax), -MathF.Sin(camerax) }, { 0, MathF.Sin(camerax), MathF.Cos(camerax) } };
      
      let rotationy:Vec<Vec<f64>> = vec![
      vec![camera_rotation_y.cos(),0.0,-camera_rotation_y.sin()],
      vec![0.0,1.0,0.0],
      vec![camera_rotation_y.sin(),0.0,camera_rotation_y.cos()],
      ];
      let rotationx:Vec<Vec<f64>> = vec![
      vec![1.0,0.0,0.0],
      vec![0.0,camera_rotation_x.cos(),-camera_rotation_x.sin()],
      vec![0.0,camera_rotation_x.sin(),camera_rotation_x.cos()],
      ];

      let half_window_hight:f64 = window_hight/2.0;
      let half_window_width:f64 = window_width/2.0;
// display triangles
  for triangle in triangles3d
  {
    
    let mut rotated_point1:Vec<f64>= multiply(&rotationx,vec![triangle.triangle_point1.x, triangle.triangle_point1.y, triangle.triangle_point1.z]);
    rotated_point1 =multiply(&rotationy,vec![rotated_point1[0], rotated_point1[1], rotated_point1[2]]);

    let mut rotated_point2:Vec<f64>= multiply(&rotationx,vec![triangle.triangle_point2.x, triangle.triangle_point2.y, triangle.triangle_point2.z]);
    rotated_point2 =multiply(&rotationy,vec![rotated_point2[0], rotated_point2[1], rotated_point2[2]]);

    let mut rotated_point3:Vec<f64>= multiply(&rotationx,vec![triangle.triangle_point3.x, triangle.triangle_point3.y, triangle.triangle_point3.z]);
    rotated_point3 =multiply(&rotationy,vec![rotated_point3[0], rotated_point3[1], rotated_point3[2]]);


   //multiply and display triangle
  



    //cheacking if it is facing the camera
    if(is_facing_camera(&calculate_triangle_normal(&point3d{x:rotated_point1[0],y:rotated_point1[1],z:rotated_point1[2]},
      &point3d{x:rotated_point2[0],y:rotated_point2[1],z:rotated_point2[2]},
      &point3d{x:rotated_point3[0],y:rotated_point3[1],z:rotated_point3[2]}),
      &Vector3::new(camera_position.x, camera_position.y, camera_position.z))){
     

      let point1:Vec<f64> = multiply(&projection,vec![rotated_point1[0], rotated_point1[1], rotated_point1[2]]);
      let point2:Vec<f64> = multiply(&projection,vec![rotated_point2[0], rotated_point2[1], rotated_point2[2]]);
      let point3:Vec<f64> = multiply(&projection,vec![rotated_point3[0], rotated_point3[1], rotated_point3[2]]);
    


    //display triangle
    let vertices: [[f64; 2]; 3] = [[point1[0]+half_window_width, point1[1]+half_window_hight], [point2[0]+half_window_width, point2[1]+half_window_hight], [point3[0]+half_window_width, point3[1]+half_window_hight]];
    let mut color = [triangle.color[0], triangle.color[1], triangle.color[2], triangle.color[3]];
    if*(triangle.lighting_on){
    let lighting:f32 = calculate_lighting(
      &calculate_triangle_normal(&point3d{x:rotated_point1[0],y:rotated_point1[1],z:rotated_point1[2]},
        &point3d{x:rotated_point2[0],y:rotated_point2[1],z:rotated_point2[2]},
        &point3d{x:rotated_point3[0],y:rotated_point3[1],z:rotated_point3[2]}),
      &Vector3::new(camera_position.x, camera_position.y, camera_position.z),
      &Vector3::new(10.0, 0.0, 0.0));

      color = [0.0*lighting, 1.0*lighting, 0.0*lighting, 1.0*lighting];
    
      }
    
    polygon(color, &vertices, c.transform, g);
    }

}
}


fn distance_to_point(point: &point3d, triangle: &triangle_points3d) -> f64 {
    // Calculate the centroid of the triangle
    let centroid = point3d {
      x: (triangle.triangle_point1.x + triangle.triangle_point2.x + triangle.triangle_point3.x) / 3.0,
      y: (triangle.triangle_point1.y + triangle.triangle_point2.y + triangle.triangle_point3.y) / 3.0,
      z: (triangle.triangle_point1.z + triangle.triangle_point2.z + triangle.triangle_point3.z) / 3.0,
    };
  
    // Calculate the distance between the point and the centroid
    let distance_vec = point3d {
      x: point.x - centroid.x,
      y: point.y - centroid.y,
      z: point.z - centroid.z,
    };
  
    // Use the distance formula (square root of sum of squared differences)
    let distance_squared = distance_vec.x * distance_vec.x + distance_vec.y * distance_vec.y + distance_vec.z * distance_vec.z;
    distance_squared.sqrt()
  }

fn multiply(matrix: &Vec<Vec<f64>>, vector: Vec<f64>) -> Vec<f64> {

   let mut result: Vec<f64> = vec![0.0; matrix.len()];
   
   for i in 0..= matrix.len() - 1 {
       let mut x: f64 = 0.0;
       for j in 0..= vector.len() - 1 {
           x = (vector[j] * matrix[i][j]) + x;
       }
       result[i] = x;
   }
   return result;
}





fn calculate_triangle_normal(v1: &point3d, v2: &point3d, v3: &point3d) -> point3d {
    // Calculate vectors along two edges of the triangle
    let edge1 = v2.sub(&v1);
    let edge2 = v3.sub(&v1);
  
    // Calculate the normal vector using the cross product
    
    let normal = Vector3::new(edge1.x, edge1.y, edge1.z).cross(&Vector3::new(edge2.x, edge2.y, edge2.z));
  
    // Normalize the normal vector
   let normalized= normal.normalize();
  
   point3d{x:normalized.x,y:normalized.y,z:normalized.z}
  }


fn is_facing_camera(triangle_normal: &point3d, cam_position: &Vector3<f64>) -> bool {
    // Normalize both vectors for accurate dot product
    let normalized_normal = Vector3::new(triangle_normal.x, triangle_normal.y, triangle_normal.z).normalize();
    let camera_direction = *cam_position - Vector3::zeros();
    let normalized_direction = camera_direction.normalize();
    
    // Check if the dot product is positive (acute angle between vectors)
    normalized_normal.dot(&normalized_direction) > 0.0
  }

  fn calculate_lighting(triangle_normal: &point3d, cam_position: &Vector3<f64>, light_direction: &Vector3<f64>) -> f32 {
    if !is_facing_camera(triangle_normal, cam_position) {
      return 0.0;
    }
  
    // Normalize vectors again for accurate lighting calculation
    let normalized_normal = Vector3::new(triangle_normal.x, triangle_normal.y, triangle_normal.z).normalize();
    let normalized_light_direction = light_direction.normalize();
  
    // Dot product to get the cosine of the angle between normal and light direction
    let cosine_theta = normalized_normal.dot(&normalized_light_direction);
  
    // Clamp cosine_theta to avoid artifacts (e.g., due to floating-point precision errors)
    let clamped_cosine = cosine_theta.clamp(0.0, 1.0);
  
    // Lighting level is proportional to the clamped cosine of the angle
    clamped_cosine as f32
  }