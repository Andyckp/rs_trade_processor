# rs_trade_processor: A Trade Enricher Application in Rust  

## Overview  
`rs_trade_processor` is a Rust-based application designed to **process and enrich raw trade data** by integrating additional information (e.g., instrument data). While the project currently includes **data structures and business logic stubs**, it provides a **modular architecture** for building a fully functional trade enrichment system.  

Although the project does not yet implement full business logic, it establishes the **core architectural components** necessary for development. 

---

## Learning Objectives  
The primary goal of this project is to explore **Rust’s capabilities**, particularly its **low-level functionalities**, including:  

1. **Struct-Based, Data-Oriented Programming** – Optimizing data structures for efficient processing.  
2. **Multi-Threading & Message Exchange** – Utilizing **MPMC ring buffers** and **duty loops**, whether partitioned by **functional unit** or **data key**.  
3. **Efficient Transport Mechanisms** – Investigating **multicast, shared memory**, and **binary codecs** for high-performance data exchange.  
4. **Macro-Based Compile-Time Programming** – Leveraging **static dispatch** for optimized execution.  

---

## Current Architecture  
1. Subscribes to **raw trade data** and multiple input data types.  
2. Stores data in **ring buffers**, categorized by type.  
3. Publishes **enriched trades** using **multi-threading**, partitioned by asset ID.  

---

## Usage
1. Run the test in trade_processor
2. Run the tests about aeron, channel and disruptor "tests" 

---

## Next Steps  
The following enhancements are planned to further develop the system:  

1. **Integrate Aeron & SBE** – Implement high-performance messaging and serialization.  
2. **Evaluate Shared Memory Usage** – Assess inter-process communication efficiency.  
3. **Compare Disruptor vs Custom Ring Buffer** – Analyze functionality and performance trade-offs.  
4. **Implement Cache-Line Padding** – Reduce **false sharing** in multi-threaded environments.  
