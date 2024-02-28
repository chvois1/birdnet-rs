use bytes::Buf;
use hound;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::result::Result;
use std::time::Instant;
use wasi_nn::{ExecutionTarget, GraphBuilder, GraphEncoding, TensorType};

/// The service handler, which receives a Request, routes on its
/// path, and returns a Future of a Response.
/// ML capabilities can be accessed through wasi-nn's core abstractions: backends, graphs, and tensors.
/// A user selects a backend for inference and loads a model, instantiated as a graph, to use in the backend.
/// Then, the user passes tensor inputs to the graph, computes the inference, and retrieves the tensor outputs.
/// The wasi-nn specification expects users to:
/// - load a model using one or more opaque byte arrays
/// - init_execution_context and bind some tensors to it using set_input
/// - compute the ML inference using the bound context
/// - retrieve the inference result tensors using get_output
async fn classify(req: Request<Body>) -> Result<Response<Body>, anyhow::Error> {
    let model_data: &[u8] =
        include_bytes!("models/BirdNET/BirdNET_GLOBAL_6K_V2.4_Model_FP32.tflite");
    let labels = include_str!("models/BirdNET/labels_fr.txt");
    let graph = GraphBuilder::new(GraphEncoding::TensorflowLite, ExecutionTarget::CPU)
        .build_from_bytes(&[model_data])?;
    let mut ctx = graph.init_execution_context()?;

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /classify such as: `curl http://localhost:8081/classify -X POST --data-binary '@soundscape.wav'`",
        ))),

        // Try to read a WAVE data from a buffer of Bytes
        // Print info about the WAVE data.
        (&Method::POST, "/classify") => {
            let buf = hyper::body::to_bytes(req.into_body()).await?;
            let b = buf.reader();
            let mut rdr = hound::WavReader::new(b) ?;
            let spec = rdr.spec();
            println!("Sample rate - the number of sample per second: {}" , spec.sample_rate);
            println!("Bits per sample - the number of bits per sample: {}", spec.bits_per_sample);
            println!("Channels - the number of channels: {}", spec.channels);
            println!("Sample format - Whether the wav's samples are float or integer values: {}", spec.channels);
            let v : Vec<f32>  = rdr.samples::<i16>()
                .filter_map(|x| x.ok())
                .map (|x| x as f32)
                .map(|x| x / 32768.0)
                .collect();
            println!("v len: {}", v.len());

            // Making a a Vec of slices into the original data. 
            let k: Vec<&[f32]> = v.chunks(3*48000)
                .collect();
            println!("k len: {}", k.len());

            let mut results = Vec::new();

            println!("[Range]\tElapsed time\tConfidence\tSpecies\tCommon name");
            for (pos, tensor_data ) in k.iter().enumerate() {
                ctx.set_input(0, TensorType::F32, &[1, 144000], tensor_data)?;

                // Execute the inference.
                let now = Instant::now();
                ctx.compute()?;
                let elapsed = now.elapsed();

                // Retrieve the output.
                let mut output_buffer = vec![0f32; labels.lines().count()];
                _ = ctx.get_output(0, &mut output_buffer)?;

                // Apply a custom sigmoid function to the prediction values
                // to get the confidence values
                let confidence : Vec<f32>= output_buffer.iter()
                    .map(|x| sigmoid(*x))
                    .collect();

                // Sort the results with the highest probability result first
                let v = sort_results(&confidence);

                // The first result's first element points to the labels position
                let class_name = labels.lines().nth(v[0].0).unwrap_or("Unknown");
                println!("[{}]\t{:.2?}\t{:.2}%\t{}\t{}",pos, elapsed, 100.0*v[0].1, v[0].0, class_name);
                results.push(v.into_iter().nth(0).unwrap());

            }
            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            let v: Vec<&InferenceResult> = results
                .iter()
                .filter(|x| x.1 > 0.8 )
                .collect();
            for r in v {
                let class_name = labels.lines().nth(r.0).unwrap_or("Unknown");
                println!("\t{}\t\t{}", class_name, r.1);    
            }
            println!("Yet another run - we hope you enjoy it !");                   
            Ok(Response::new(Body::from("Yet another run - we hope you enjoy it !")))
        }
        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
    let make_svc =
        make_service_fn(
            |_| async move { Ok::<_, Infallible>(service_fn(move |req| classify(req))) },
        );
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}

fn sigmoid(x: f32) -> f32 {
    1 as f32 / (1 as f32 + std::f32::consts::E.powf(-x))
}

// Sort the buffer of probabilities. The graph places the match probability for each class at the
// index for that class (e.g. the probability of class 42 is placed at buffer[42]). Here we convert
// to a wrapping InferenceResult and sort the results.
fn sort_results(buffer: &[f32]) -> Vec<InferenceResult> {
    let mut results: Vec<InferenceResult> = buffer
        .iter()
        .enumerate()
        .map(|(c, p)| InferenceResult(c, *p))
        .collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

// A wrapper for class ID and match probabilities.
struct InferenceResult(usize, f32);
