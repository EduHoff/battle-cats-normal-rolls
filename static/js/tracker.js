document.addEventListener("DOMContentLoaded", () => {
    const container = document.querySelector(".tracker-container");
    if (!container) return;

    const tableA = container.querySelector(".track-a-table");
    const tableB = container.querySelector(".track-b-table");

    if (!tableA || !tableB) return;

    container.addEventListener("click", (e) => {
        if (e.target.tagName === "A") return;

        const targetCell = e.target.closest("td.pick");
        if (!targetCell) return;

        const isLastPicked = targetCell.classList.contains("last-picked");

        container.querySelectorAll(".picked, .next_position, .last-picked").forEach((el) => {
            el.classList.remove("picked", "next_position", "last-picked");
        });

        if (isLastPicked) return;

        const rowsA = Array.from(tableA.querySelectorAll("tbody tr:not(.dummy-row)"));
        const rowsB = Array.from(tableB.querySelectorAll("tbody tr:not(.dummy-row)"));

        const totalSteps = Math.min(rowsA.length, rowsB.length);
        const clickedBottomCell = e.target.closest("td.bottom-cell");

        let currentTrack = "A";
        let pathFound = false;

        for (let i = 0; i < totalSteps; i++) {
            const cellInPath = getCellForStep(i, currentTrack, rowsA, rowsB);
            if (!cellInPath) continue;

            cellInPath.classList.add("picked");

            if (cellInPath === targetCell) {
                pathFound = true;
                targetCell.classList.add("last-picked");

                const isSwitchTriggered = clickedBottomCell
                    ? hasTrackSwitch(clickedBottomCell)
                    : !targetCell.querySelector(".top-cell") && hasTrackSwitch(targetCell);

                if (isSwitchTriggered) {
                    currentTrack = currentTrack === "A" ? "B" : "A";
                }

                highlightNextPosition(i + 1, currentTrack, rowsA, rowsB);
                break;
            }

            if (hasTrackSwitch(cellInPath)) {
                currentTrack = currentTrack === "A" ? "B" : "A";
            }
        }

        if (!pathFound) {
            container
                .querySelectorAll(".picked, .last-picked")
                .forEach((el) => el.classList.remove("picked", "last-picked"));

            let isolatedTrack = tableA.contains(targetCell) ? "A" : "B";

            let stepIndex = -1;
            if (isolatedTrack === "A") {
                stepIndex = rowsA.findIndex((tr) => tr.contains(targetCell));
            } else {
                stepIndex = rowsB.findIndex((tr) => tr.contains(targetCell));
            }

            targetCell.classList.add("picked", "last-picked");

            const isSwitchTriggered = clickedBottomCell
                ? hasTrackSwitch(clickedBottomCell)
                : !targetCell.querySelector(".top-cell") && hasTrackSwitch(targetCell);

            if (isSwitchTriggered) {
                isolatedTrack = isolatedTrack === "A" ? "B" : "A";
            }

            if (stepIndex !== -1) {
                highlightNextPosition(stepIndex + 1, isolatedTrack, rowsA, rowsB);
            }
        }
    });

    function getCellForStep(stepIndex, track, rowsA, rowsB) {
        const targetRow = track === "A" ? rowsA[stepIndex] : rowsB[stepIndex];
        return targetRow ? targetRow.querySelector("td.pick") : null;
    }

    function hasTrackSwitch(element) {
        const text = element.textContent;
        return text.includes("->") || text.includes("<-");
    }

    function highlightNextPosition(nextStepIndex, track, rowsA, rowsB) {
        const targetCell = getCellForStep(nextStepIndex, track, rowsA, rowsB);
        if (targetCell) {
            targetCell.classList.add("next_position");
        }
    }
});
