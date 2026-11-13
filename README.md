# Data Structures in Rust

Different data structures and algorithms implemented from scratch in Rust, organized as a Cargo workspace.

## Project Structure

* `gap_buffer` — Gap buffer text editor data structure
* `tailed_linked_list` — Singly linked list with head and tail pointers using manual memory management
* `mesh` — 3D triangle mesh processing and spatial distance queries
* `graph` — Mesh adjacency list and Breadth-First Search (BFS) traversal

## Usage

Each workspace member is a standalone binary crate and can be run from the root directory using the `-p` (package) flag:

```bash
cargo run -p [workspace_member]
```
