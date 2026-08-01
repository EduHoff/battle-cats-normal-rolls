# Battle Cats Normal Seed Tracker

Battle Cats normal seed tracking server with high-performance Rust seeker

## About This Tool & Important Notes

> **Important:**
> The normal gacha **does NOT share a seed with the rare gacha**. Your rare seed will not work here. You must find and track your normal gacha seed separately.

### Key Features & Mechanics

* **Track Switches:**  
  Track switches in the normal gacha work the exact same way as in rare banners. You can strategically roll duplicate units to trigger or avoid track switches.

* **Interactive Path Simulation:**  
Clicking on any cell in the table simulates your roll path from the top. Previous rolls along the path are marked as consumed (`.picked`), taking into account any duplicate track switches, and your very next available roll is highlighted (`.next_position`) for precise roll tracking.

* **What is `100K XP (β)`?**  
  The *Lucky Ticket G* banner uses a special variant of the 100K XP item with a different internal ID from the standard 100K XP used in other banners.  
  * While there is no difference in-game, this ID distinction prevents it from being used to force track switches across different banners.
  * To distinguish between the two in the tracker, the *Lucky Ticket G*-exclusive version is marked with **(β)**.

## Installation & Building

### Prerequisites

To build this project from source, ensure you have the following installed on your system:
* [Git](https://git-scm.com/)
* [Rust & Cargo](https://www.rust-lang.org/) (latest stable release recommended)

---

### Installing via AUR (Arch Linux)

If you are using Arch Linux or an Arch-based distribution, you can install the package directly from the AUR using your favorite helper:

```bash
yay -S battle-cats-normal-rolls-git
```

---

### Running via Docker

If you prefer not to install the Rust toolchain, you can build and run the application using Docker and Docker Compose:

```bash
git clone https://github.com/EduHoff/battle-cats-normal-rolls.git
cd battle-cats-normal-rolls
docker compose up --build
```

The server will be available at http://localhost:3000

To stop the container:

```bash
docker compose down
```

---

### Building from Source

To compile and run the project manually:

```bash
git clone https://github.com/EduHoff/battle-cats-normal-rolls.git
cd battle-cats-normal-rolls
```

Build for production:

```bash
cargo build --release
```

Run the application:
```bash
./target/release/battle-cats-normal-rolls
```

## Credits & Acknowledgments

This project is a complete rewrite in Rust, inspired by and building upon the work of the following open-source projects:

* **[bc-normal-seed-tracking](https://github.com/ampuri/bc-normal-seed-tracking)** by **Amp (ampuri)**  
  Original inspiration for specialized normal gacha seed tracking logic and mechanics.

* **[battle-cats-rolls](https://gitlab.com/godfat/battle-cats-rolls)** by **Lin Jen-Shin (godfat)**  
  The foundational seed tracking system, UI layout, and the base `tacit.css` stylesheet used for styling this application.

## License

* The core source code of this repository is licensed under the **GNU Affero General Public License v3.0 (AGPLv3)**.
* The static CSS file (`static/css/style.css`) includes modified code derived from `battle-cats-rolls` and retains its original **Apache License, Version 2.0**.

See the `LICENSE` file in this repository for full license details.
