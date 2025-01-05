# practice-rust-mpsc

<!--toc:start-->

- [practice-rust-mpsc](#practice-rust-mpsc)
  - [Problem](#problem)
    - [Scenario](#scenario)
    - [Requirements](#requirements)
  - [Follow-up Questions](#follow-up-questions)
  - [Expected Answer](#expected-answer)
  <!--toc:end-->

MPSC N Producer, M consumer examples in Rust

---

## Problem

Using **Channels** and **Multi-threading** in Rust, write a program that meets the following requirements:

### Scenario

- The program should create **N Producer threads** and **M Consumer threads**.
- Each Producer thread generates a random number every **1 second** and sends it to a message queue.
- Each Consumer thread reads values from the message queue, processes them (e.g., calculates the square), and prints the result.
- The program should run for **10 seconds** and, upon termination, print the total number of messages generated and processed.

### Requirements

1. Use Rust’s `std::sync::mpsc` channels to implement message passing between Producers and Consumers.
2. Each Producer and Consumer should operate in separate threads.
3. Consumers should wait if the message queue is empty.
4. Ensure thread safety and avoid concurrency issues.
