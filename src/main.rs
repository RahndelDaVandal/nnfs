use nnfs_tmp::{
    nueron::Nueron,
    layer::Layer
};

fn main() {
    // TODO - use env_logger pipe to log file
    env_logger::init();

    let inputs: Vec<f32> = vec!(1.0, 2.0, 3.0, 2.5);

    let n1 = Nueron::new(vec![0.2, 0.8, -0.5, 1.0], 2.0);
    let n2 = Nueron::new(vec![0.5, -0.91, 0.26, -0.5], 3.0);
    let n3 = Nueron::new(vec![-0.26, -0.27, 0.17, 0.87], 0.5);

    let layer = Layer::new(vec![n1, n2, n3]);

    let output: Vec<f32> = layer.output(&inputs);
    println!("{:?}", output);
}

