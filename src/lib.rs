// src/lib.rs
/*
 * Core library for GasOptimizer
 */

use log::{info, error, debug};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

/// Custom result type to handle errors with a boxed trait object
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize)]
/// Process result structure
pub struct ProcessResult {
    /// Whether the process was successful
    pub success: bool,
    /// Process message
    pub message: String,
    /// Optional process data
    pub data: Option<serde_json::Value>,
}

#[derive(Debug)]
/// GasOptimizer processor structure
pub struct GasOptimizerProcessor {
    /// Verbose mode flag
    verbose: bool,
    /// Processed item count
    processed_count: usize,
}

impl GasOptimizerProcessor {
    /// Creates a new GasOptimizer processor
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            processed_count: 0,
        }
    }

    /// Processes the given data
    pub fn process(&mut self, data: &str) -> Result<ProcessResult> {
        if self.verbose {
            debug!("Processing data of length: {}", data.len());
        }

        // Simulate processing
        self.processed_count += 1;
        
        let result = ProcessResult {
            success: true,
            message: format!("Successfully processed item #{}", self.processed_count),
            data: Some(serde_json::json!({
                "length": data.len(),
                "processed_at": chrono::Utc::now().to_rfc3339(),
                "item_number": self.processed_count
            })),
        };

        Ok(result)
    }

    /// Returns processor statistics as JSON
    pub fn get_stats(&self) -> serde_json::Value {
        serde_json::json!({
            "processed_count": self.processed_count,
            "verbose": self.verbose
        })
    }
}

/// Main processing function
pub fn run(verbose: bool, input: Option<String>, output: Option<String>) -> Result<()> {
    // Initialize logger
    if verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }
    
    info!("Starting GasOptimizer processing");
    
    let mut processor = GasOptimizerProcessor::new(verbose);
    
    // Read input
    let input_data = match input {
        Some(path) => {
            if let Ok(data) = fs::read_to_string(path) {
                data
            } else {
                error!("Failed to read input file: {}", path.display());
                return Err("Failed to read input file".into());
            }
        },
        None => {
            error!("No input file provided");
            return Err("No input file provided".into());
        }
    };
    
    // Process input data
    let result = processor.process(&input_data);
    
    // Handle processing result
    match result {
        Ok(process_result) => {
            info!("Processing result: {:?}", process_result);
            // Save result to output file if specified
            if let Some(output_path) = output {
                if let Ok(data) = serde_json::to_string(&process_result) {
                    if let Ok(_) = fs::write(output_path, data) {
                        info!("Result saved to output file: {}", output_path.display());
                    } else {
                        error!("Failed to save result to output file: {}", output_path.display());
                    }
                } else {
                    error!("Failed to serialize processing result");
                }
            }
        },
        Err(err) => {
            error!("Processing error: {}", err);
            return Err(err);
        }
    }
    
    Ok(())
}