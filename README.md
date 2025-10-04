<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool.

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos --locked`

Then run

`cargo leptos new --git leptos-rs/start-actix`

to generate a new project template (you will be prompted to enter a project name).

`cd {projectname}`

to go to your newly created project.

Of course, you should explore around the project structure, but the best place to start with your application code is in `src/app.rs`.

## Running your project

`cargo leptos watch`  
By default, you can access your local project at `http://localhost:3000`

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future)

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
leptos_start
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="leptos_start"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```
Finally, run the server binary.

## Notes about CSR and Trunk:
Although it is not recommended, you can also run your project without server integration using the feature `csr` and `trunk serve`:

`trunk serve --open --features csr`

This may be useful for integrating external tools which require a static site, e.g. `tauri`.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.

# 🧩 Letpos Staff Dashboard Project

This project demonstrates a complete **CRUD (Create, Read, Update, Delete)** flow for managing team members using the **Leptos** web framework.  
It highlights and keeps records of **Frontend Engineers**, **Backend Engineers**, and **UI/UX Designers** collaborate effectively working in an organization.

---

### 🧠 Core Data Model

```rust
Person {
    uuid: Uuid,
    name: String,
    title: String,
    level: String,
    compensation: f64,
    joined_date: NaiveDate,
}
 ```
Each `Person` represents a team member with details such as **name**, **job title**, **level**, **salary**, and **date of joining**.

---

## 🧩 Frontend Engineers

Frontend engineers focus on building the **interactive user interface** using **Leptos’ reactive components**.

They are responsible for implementing:

- 📝 **Data forms** for creating and editing `Person` entries.  
- 📋 **Dynamic lists** to display team members with real-time updates.  
- ✅ **Client-side validation** using the `validator` crate and Leptos signals.  
- 📊 **Integration of charts** using `charts-rs` to visualize compensation or team growth trends.

These features ensure a seamless user experience, where frontend components react instantly to user input and data changes.

---

## 🎨 UI/UX Designers

UI/UX designers ensure that the interface is **intuitive**, **accessible**, and **visually consistent**.  
They collaborate by:

- 🧱 **Creating design mockups and wireframes** for CRUD interfaces.  
- 🔤 **Defining the visual hierarchy and component layout** for readability.  
- 🎨 **Using Tailwind CSS or custom CSS** for clean, responsive styling.  
- 🚀 **Ensuring a smooth user journey** from adding new team members to editing or removing them.

Their work helps maintain a cohesive design system and improves usability across devices and workflows.

---

## 🚀 Example Use Cases

This project can be adapted for a variety of real-world scenarios, such as:

- 🧾 **HR dashboard** for managing employee data.  
- 💼 **Startup team overview** displaying compensation and growth analytics.  
- 🔍 **Real-time team directory** with filtering and sorting capabilities.

These examples illustrate how Leptos can power dynamic, data-driven interfaces with real-time interactivity.

---

## 🧰 Tech Stack

This project leverages a modern Rust-based full-stack architecture, including:

- ⚛️ **Leptos** – Reactive frontend and server-side rendering (SSR) framework  
- 🚀 **Actix Web** – High-performance backend web server  
- 🧮 **SurrealDB** – Modern, flexible database for structured and unstructured data  
- 🎨 **Tailwind CSS** – Utility-first styling framework for rapid UI development  
- 📊 **Charts-rs** – Data visualization library for analytics and insights  
- 🧾 **UUID / Chrono** – For unique identifiers and robust date-time handling

---

### 🧩 Supporting Libraries & Utilities
- 🧾 **UUID / Chrono** – Unique identifiers and robust date-time handling  
- 🧠 **Validator** – Type-safe validation for user input and form data  
- ⚙️ **Serde** – Serialization and deserialization framework  
- 🪶 **Wasm-bindgen** – WebAssembly bindings for Rust-to-JS interop  
- 🧩 **Leptos Router / Meta / Actix** – For routing, metadata management, and SSR integration  
- 🔧 **ThisError / OnceCell / Cfg-if** – Utilities for error handling, lazy initialization, and conditional compilation  
- 🔢 **Num-format** – For locale-aware number formatting 
