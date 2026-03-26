use jni::objects::{JClass, JString};
use jni::sys::{jdouble, jlong};
use jni::JNIEnv;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use blake3::Hasher;

static TOTAL_OPERATIONS: AtomicU64 = AtomicU64::new(0);
static TOTAL_LATENCY_NS: AtomicU64 = AtomicU64::new(0);

lazy_static::lazy_static! {
    static ref REALITY_CACHE: DashMap<String, RealityState> = DashMap::new();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealityState {
    pub value: f64,
    pub confidence: f64,
    pub entropy: f64,
    pub timestamp: u128,
    pub integrity_hash: [u8; 32],
}

pub fn calculate_reality(intent: f64, statement: &str, timestamp: u128) -> RealityState {
    let start_time = Instant::now();
    let weight = statement.len() as f64;
    let entropy = calculate_entropy(statement);
    let phi = 0.6180339887498948482;
    let beta = 0.001;
    let time_factor = (-beta * (timestamp as f64)).exp();
    let mutual_info = calculate_mutual_information(statement);
    
    let base_value = intent * weight * phi;
    let enhanced_value = base_value * time_factor * (1.0 + mutual_info);
    let confidence = (entropy * 0.5).min(1.0).max(0.0);
    
    let mut hasher = Hasher::new();
    hasher.update(&intent.to_be_bytes());
    hasher.update(statement.as_bytes());
    hasher.update(&timestamp.to_be_bytes());
    let integrity_hash = hasher.finalize();
    
    let latency = start_time.elapsed();
    TOTAL_OPERATIONS.fetch_add(1, Ordering::SeqCst);
    TOTAL_LATENCY_NS.fetch_add(latency.as_nanos() as u64, Ordering::SeqCst);
    
    RealityState {
        value: enhanced_value,
        confidence,
        entropy,
        timestamp,
        integrity_hash: *integrity_hash.as_bytes(),
    }
}

fn calculate_entropy(statement: &str) -> f64 {
    if statement.is_empty() { return 0.0; }
    let mut freq = [0u32; 256];
    let total = statement.len() as f64;
    for byte in statement.bytes() { freq[byte as usize] += 1; }
    -freq.iter().filter(|&&count| count > 0).map(|&count| {
        let p = count as f64 / total;
        p * p.log2()
    }).sum::<f64>()
}

fn calculate_mutual_information(statement: &str) -> f64 {
    let unique_chars = statement.chars().collect::<std::collections::HashSet<_>>().len();
    let total_chars = statement.len();
    if total_chars == 0 { return 0.0; }
    (unique_chars as f64 / total_chars as f64) * 0.1
}

#[no_mangle]
pub extern "system" fn Java_com_realityengine_NativeEngine_processStatement(
    env: JNIEnv,
    _class: JClass,
    intent: jdouble,
    statement: JString,
) -> jdouble {
    let s_input: String = env.get_string(&statement).unwrap().into();
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
    let result = calculate_reality(intent as f64, &s_input, timestamp);
    result.value as jdouble
}
