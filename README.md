# Battle Cats Normal Seed Tracker

(no oficial description yet)

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
