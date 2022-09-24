#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            plot_logistic_map,
            simulate_dynamics
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn plot_logistic_map(
    a: f32,
    b: f32,
    delta: f32,
    r: f32,
) -> (Vec<f32>, Vec<f32>, Vec<f32>, Vec<f32>) {
    let mut res = (vec![], vec![], vec![], vec![]);
    let mut x = a;
    while x <= b {
        res.0.push(x);
        let y = logistic_map(r, x);
        res.1.push(y - x);
        res.2.push(y.powf(2.0) - x);
        res.3.push(y.powf(3.0) - x);
        x += delta;
    }
    res
}

#[tauri::command]
fn simulate_dynamics(x_0: f32, n: usize, r: f32) -> (Vec<usize>, Vec<f32>) {
    let mut res = (vec![0], vec![x_0]);
    for i in 0..n + 1 {
        res.0.push(i);
        res.1.push(logistic_map(r, *res.1.last().unwrap()));
    }
    res
}

fn logistic_map(r: f32, x: f32) -> f32 {
    r * x * (1.0 - x)
}
