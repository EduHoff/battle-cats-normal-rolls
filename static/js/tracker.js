document.addEventListener("DOMContentLoaded", () => {
    const table = document.getElementById("tracker-table");
    if (!table) return;

    table.addEventListener("click", (e) => {
        if (e.target.tagName === 'A') return;

        const targetCell = e.target.closest("td.pick");
        if (!targetCell) return;

        table.querySelectorAll(".picked, .next_position").forEach(el => {
            el.classList.remove("picked", "next_position");
        });

        const rows = Array.from(table.querySelectorAll("tbody tr"));
        const clickedBottomCell = e.target.closest("td.bottom-cell");

        let currentTrack = 'A';
        let pathFound = false;

        for (let r = 0; r < rows.length; r++) {
            const row = rows[r];
            const cellInPath = getCellForTrack(row, currentTrack);

            if (!cellInPath) continue;

            cellInPath.classList.add("picked");

            if (cellInPath === targetCell) {
                pathFound = true;

                const isSwitchTriggered = clickedBottomCell 
                    ? hasTrackSwitch(clickedBottomCell)
                    : (!targetCell.querySelector(".top-cell") && hasTrackSwitch(targetCell));

                if (isSwitchTriggered) {
                    currentTrack = currentTrack === 'A' ? 'B' : 'A';
                }

                highlightNextPosition(rows, r + 1, currentTrack);
                break;
            }

            if (hasTrackSwitch(cellInPath)) {
                currentTrack = currentTrack === 'A' ? 'B' : 'A';
            }
        }

        if (!pathFound) {
            table.querySelectorAll(".picked").forEach(el => el.classList.remove("picked"));

            const targetRowIdx = rows.indexOf(targetCell.parentElement);
            const allCells = Array.from(targetCell.parentElement.cells);
            let isolatedTrack = allCells.indexOf(targetCell) < Math.floor(allCells.length / 2) ? 'A' : 'B';

            targetCell.classList.add("picked");

            const isSwitchTriggered = clickedBottomCell 
                ? hasTrackSwitch(clickedBottomCell)
                : (!targetCell.querySelector(".top-cell") && hasTrackSwitch(targetCell));

            if (isSwitchTriggered) {
                isolatedTrack = isolatedTrack === 'A' ? 'B' : 'A';
            }

            highlightNextPosition(rows, targetRowIdx + 1, isolatedTrack);
        }
    });

    function getCellForTrack(row, track) {
        const cells = Array.from(row.querySelectorAll("td.pick"));
        return track === 'A' ? cells[0] : (cells[1] || cells[0]);
    }

    function hasTrackSwitch(element) {
        return element.textContent.includes("->");
    }

    function highlightNextPosition(rows, startRowIdx, track) {
        if (startRowIdx < rows.length) {
            const targetCell = getCellForTrack(rows[startRowIdx], track);
            if (targetCell) {
                targetCell.classList.add("next_position");
            }
        }
    }
});