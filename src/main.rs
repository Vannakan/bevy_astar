
use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

const WIDTH: f32 = 50.0;
const HEIGHT: f32 = 50.0;
const GRID_HEIGHT: i32 = 50;
const GRID_WIDTH: i32 = 50;
const ZOOM: f32 = 4.0;

#[derive(Clone, Copy)]
pub struct Node{ x: f32, y: f32, f: f32, g:f32, h: f32, accessible: bool } // every node needs a reference to previous node (box)

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Resource)]
pub struct Graph {
    pub nodes: Vec<Vec<Node>>
}
#[derive(Resource)]
pub struct ClosedSet {
    pub nodes: Vec<Node>
}

#[derive(Resource)]
pub struct OpenSet {
    pub nodes: Vec<Node>
}

#[derive(Resource)]
pub struct Goal {
    pub node: Option<Node>
}

#[derive(Resource)]
pub struct Current {
    pub node: Option<Node>
}

#[derive(Resource)]
pub struct Start {
    pub node: Option<Node>
}

#[derive(Resource)]
pub struct Neighbours {
    pub nodes: Vec<Node>
}

#[derive(Resource)]
pub struct Path {
    pub nodes: Vec<Node>
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Startup, setup_nodes.after(setup))
    //.add_systems(Startup, setup_open_set.after(setup_nodes))
    .add_systems(Update, mouse_click)
    .add_systems(Update, reset_grid)
    .add_systems(Update, draw_nodes)
    .add_systems(Update, draw_path)
    .add_systems(Update, find_goal.after(draw_nodes))//change
    .insert_resource(Graph{ nodes: Vec::new()})
    .insert_resource(OpenSet{ nodes: Vec::new()})
    .insert_resource(ClosedSet{ nodes: Vec::new()})
    .insert_resource(Path{ nodes: Vec::new()})
    .insert_resource(Start{ node: None})
    .insert_resource(Goal{ node: None})
    .insert_resource(Current{ node: None})
    .run();
}

#[derive(Component)]
pub struct Camera;

pub fn mouse_click(mouse: Res<Input<MouseButton>>, mut start: ResMut<Start>, mut goal: ResMut<Goal>, q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&bevy::render::camera::Camera, &GlobalTransform)>,mut graph: ResMut<Graph>, mut open_set: ResMut<OpenSet>, mut current: ResMut<Current>
){
    if mouse.just_released(MouseButton::Left)
    { 
        let (camera, camera_transform) = camera_q.single();
        if let Some(position) = q_windows.single().cursor_position()
        {
            let pos = camera.viewport_to_world_2d(camera_transform, position).unwrap();
            if pos.x as i32 >= 0 && pos.x as i32 <= (GRID_WIDTH - 1) * WIDTH as i32 && pos.y as i32 >= 0 && pos.y as i32 <= (GRID_HEIGHT - 1) * HEIGHT as i32{
                let x = f32::round(pos.x  / WIDTH) as usize;
                let y = f32::round(pos.y/ HEIGHT) as usize;
                println!("{},{}", x,y);
                if let None = start.node{
              

                    open_set.nodes.push(graph.nodes[x][y]);
                    start.node = Some(graph.nodes[x][y]);
                    current.node = start.node;
                
                }else if let None = goal.node{
                    goal.node = Some(graph.nodes[x][y]);
                }
            }

        }

    } else  if mouse.pressed(MouseButton::Right)
    { 
        let (camera, camera_transform) = camera_q.single();
        if let Some(position) = q_windows.single().cursor_position()
        {
            let pos = camera.viewport_to_world_2d(camera_transform, position).unwrap();
            if pos.x as i32 >= 0 && pos.x as i32 <= (GRID_WIDTH - 1) * WIDTH as i32 && pos.y as i32 >= 0 && pos.y as i32 <= (GRID_HEIGHT - 1) * HEIGHT as i32{
                let x = f32::round(pos.x  / WIDTH) as usize;
                let y = f32::round(pos.y/ HEIGHT) as usize;
                graph.nodes[x][y].accessible = false;
                graph.nodes[x +1][y].accessible = false;
                graph.nodes[x + 1][y+1].accessible = false;
                graph.nodes[x-1][y-1].accessible = false;
                graph.nodes[x][y-1].accessible = false;
            }

        }
    }
}

pub fn reset_grid(input: Res<Input<KeyCode>>, mut start: ResMut<Start>, mut goal: ResMut<Goal>, mut closed_set: ResMut<ClosedSet>, mut open_set: ResMut<OpenSet>, mut current: ResMut<Current>, mut graph: ResMut<Graph>){
    if input.just_released(KeyCode::R) {
        start.node = None;
        current.node = None;
        goal.node = None;
        closed_set.nodes = Vec::new();
        open_set.nodes = Vec::new();

        for x in 0..GRID_WIDTH as usize{
            graph.nodes.push(Vec::new());
            for y in 0..GRID_HEIGHT as usize {
                graph.nodes[x][y].accessible = true;
    
            }
        }
    }
}

pub fn draw_path(mut gizmos: Gizmos, goal: Res<Goal>, path: Res<Path>){
    // if let None = goal.node {
    //     return;
    // }

    // for test in path.nodes.iter() {
    //     gizmos.rect_2d(Vec2::from([test.x * WIDTH, test.y * HEIGHT]), 0.0, Vec2::from([WIDTH, HEIGHT]), Color::PINK); 
    // }
}

pub fn setup(mut commands: Commands){
    commands.spawn((Camera2dBundle{  
        transform: Transform { translation: Vec3::from([GRID_WIDTH as f32 / 2.0  * WIDTH  , GRID_HEIGHT as f32 / 2.0 * HEIGHT , 0.0]), ..Default::default() },
        projection: OrthographicProjection {
            scale: ZOOM,
            ..Default::default()     
        },
        ..Default::default()
    },Camera));
}

pub fn setup_nodes(mut graph: ResMut<Graph>, mut start: ResMut<Start>, mut goal: ResMut<Goal>)
{
    for x in 0..GRID_WIDTH as usize{
        graph.nodes.push(Vec::new());
        for y in 0..GRID_HEIGHT as usize {
            graph.nodes[x].push(Node{x: x as f32, y: y as f32, f: 0.0, h: 0.0, g: 0.0, accessible: true});

        }
    }
}

pub fn draw_nodes(mut gizmos: Gizmos, graph: Res<Graph>, start: Res<Start>, goal: Res<Goal>, open_set: Res<OpenSet>, closed_set: Res<ClosedSet>)
{
    if graph.nodes.len() <= 0 { return; }


    for x in 0..GRID_HEIGHT as usize {
        for y in 0..GRID_WIDTH as usize {
            let node = &graph.nodes[x] [y];

            if node.accessible == true {
                gizmos.rect_2d(Vec2::from([node.x * WIDTH, node.y * HEIGHT]), 0.0, Vec2::from([WIDTH, HEIGHT]), Color::GREEN); 
            } 
      
        }
    }

    for x in 0..GRID_HEIGHT as usize {
        for y in 0..GRID_WIDTH as usize {
            let node = &graph.nodes[x] [y];

            if node.accessible == false {
                gizmos.rect_2d(Vec2::from([node.x * WIDTH, node.y * HEIGHT]), 0.0, Vec2::from([WIDTH, HEIGHT]), Color::PURPLE); 
            }
      
        }
    }
 

    
 
    if open_set.nodes.is_empty() == false {
        for n in open_set.nodes.iter() {
            gizmos.rect_2d(Vec2::from([n.x * WIDTH, n.y * HEIGHT]), 0.0, Vec2::from([WIDTH, HEIGHT]), Color::YELLOW); 
        }
    }   

    if closed_set.nodes.is_empty() == false {
        for n in closed_set.nodes.iter() {
            gizmos.rect_2d(Vec2::from([n.x * WIDTH, n.y * HEIGHT]), 0.0, Vec2::from([WIDTH, HEIGHT]), Color::BLACK); 
        }
    }


    if let None = start.node {
        return;
    }

     let start_node = start.node.unwrap();
    gizmos.rect_2d(Vec2::from([start_node.x * WIDTH, start_node.y * HEIGHT]), 0.0, Vec2::from([WIDTH, HEIGHT]), Color::RED); 
    
    if let None = goal.node {
        return;
    }


    let goal_node = goal.node.unwrap();
    gizmos.rect_2d(Vec2::from([goal_node.x * WIDTH, goal_node.y * HEIGHT]), 0.0, Vec2::from([WIDTH, HEIGHT]), Color::BLUE);     
}

fn find_best_open(mut nodes: Vec<Node>) -> usize  {
    let mut index = 0;
    for (node_index, node) in nodes.iter().enumerate() {
        if node.f <= nodes[index].f
        {
            index = node_index
        }
    }

    index
}
pub fn heuristic(current: Node, goal: Node) -> f32 {
    Vec2::from([current.x, current.y]).distance(Vec2::from([goal.x, goal.y]))
}

pub fn find_goal(mut open_set: ResMut<OpenSet>, mut closed_set: ResMut<ClosedSet>, goal_node: Res<Goal>, mut current: ResMut<Current>, graph: Res<Graph>){

    let goal = match goal_node.node {
        Some(goal) => goal,
        _ => { return ;}
    };

    let node = match current.node {
        Some(current) => current,
        _ => { return ;}
    };

    if node == goal{  
        return;         
    }
    if open_set.nodes.is_empty() {
        return;
    }

    let best = find_best_open(open_set.nodes.clone());

     current.node = Some(open_set.nodes[best].clone());

    let current_node = current.node.unwrap();
    let neighbours = get_neighbours(current_node, &graph);

    if let Some(index) = open_set.nodes.iter().position(|&x| x == current_node){
        closed_set.nodes.push(current_node);
        open_set.nodes.remove(index);
    }

    for neighbour in neighbours.iter() {
            let mut open_neighbour = neighbour.clone();

            if neighbour.accessible == false {
                let index = open_set.nodes.iter().position(|&x| x == *neighbour);
                if let Some(i) = index {
                    open_set.nodes.remove(i);
                    closed_set.nodes.push(neighbour.clone());
                }
                continue;
            }

            open_neighbour.g = current_node.g + 1.0;
            open_neighbour.h = heuristic(open_neighbour, goal);
            open_neighbour.f = open_neighbour.g + open_neighbour.h;
            
            if closed_set.nodes.iter().any(|&x| x == open_neighbour) && open_neighbour.g > neighbour.g{
                continue;
            }

            if open_set.nodes.iter().any(|&x| x == open_neighbour) && open_neighbour.g > neighbour.g {
                    continue;
            }
             
            open_set.nodes.push(open_neighbour)
    }
}

fn get_neighbours(current: Node, graph: &Res<Graph>) -> Vec<Node>{
    let c = current;
    
    let mut neighbours: Vec<Node> = Vec::new();

    // top right
    if c.x as i32 + 1 <= GRID_WIDTH - 1 && c.y as i32 + 1 <= GRID_HEIGHT - 1 {
        neighbours.push(graph.nodes[(c.x as i32 + 1) as usize] [(c.y as i32 + 1) as usize]);
    }
    // right
    if c.x as i32 + 1 <= GRID_WIDTH - 1 {
        neighbours.push(graph.nodes[(c.x as i32 + 1) as usize] [c.y as usize]);
    }
    // top left
    if c.x as i32 - 1 >= 0  && c.y as i32 + 1 <= GRID_HEIGHT - 1 {
        neighbours.push(graph.nodes[(c.x as i32 - 1) as usize] [(c.y as i32 + 1) as usize]);
    }
    // left
    if c.x as i32 - 1 >= 0 {
        neighbours.push(graph.nodes[(c.x as i32 - 1) as usize] [c.y as usize]);
    }
    // top
    if c.y as i32 + 1 <= GRID_HEIGHT - 1 {
        neighbours.push(graph.nodes[c.x as usize] [(c.y as i32 + 1) as usize] );
    }
    // bottom
    if c.y as i32 - 1 >= 0 {
        neighbours.push(graph.nodes[c.x as usize] [(c.y as i32 - 1) as usize]);
    }
    // bottom left
    if c.x as i32 - 1 >= 0  && c.y as i32 - 1 >= 0 {
        neighbours.push(graph.nodes[(c.x as i32 - 1) as usize] [(c.y as i32 - 1) as usize]);
    }
    // bottom right
    if c.x as i32 + 1 <= GRID_WIDTH - 1  && c.y as i32 - 1 >= 0 {
        neighbours.push(graph.nodes[(c.x as i32 + 1) as usize] [(c.y as i32 - 1) as usize]);
    }

    neighbours
}
