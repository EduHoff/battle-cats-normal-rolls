document.addEventListener("DOMContentLoaded", () => {
    const container = document.querySelector(".tracker-container");
    if (!container) return;

    const tableA = container.querySelector(".track-a-table");
    const tableB = container.querySelector(".track-b-table");
    if (!tableA || !tableB) return;

    container.addEventListener("click", (e) => {
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

        let currentTrack = "A";
        let pathFound = false;
        let lastItemName = null;

        for (let i = 0; i < totalSteps; i++) {
            const cellInPath = getCellForStep(i, currentTrack, rowsA, rowsB);
            if (!cellInPath) continue;

            cellInPath.classList.add("picked");

            const mainItemElem = cellInPath.querySelector(".track-link-direct");
            const switchElem = cellInPath.querySelector(".track-link-switch");

            const mainItemName = mainItemElem ? mainItemElem.textContent.trim() : "";
            const hasDuplicateReroll = switchElem !== null;

            if (cellInPath === targetCell) {
                pathFound = true;
                targetCell.classList.add("last-picked");

                const clickedOnSwitch = e.target.closest(".track-link-switch") !== null;

                if (clickedOnSwitch || (hasDuplicateReroll && lastItemName === mainItemName)) {
                    currentTrack = currentTrack === "A" ? "B" : "A";
                }

                highlightNextPosition(i + 1, currentTrack, rowsA, rowsB);
                break;
            }

            if (hasDuplicateReroll && lastItemName === mainItemName) {
                currentTrack = currentTrack === "A" ? "B" : "A";
                lastItemName = switchElem ? switchElem.textContent.replace(/->|<-|\d+[AB]/g, "").trim() : "";
            } else {
                lastItemName = mainItemName;
            }
        }

        if (!pathFound) {
            container
                .querySelectorAll(".picked, .last-picked")
                .forEach((el) => el.classList.remove("picked", "last-picked"));

            let isolatedTrack = tableA.contains(targetCell) ? "A" : "B";

            const stepIndex =
                isolatedTrack === "A"
                    ? rowsA.findIndex((tr) => tr.contains(targetCell))
                    : rowsB.findIndex((tr) => tr.contains(targetCell));

            targetCell.classList.add("picked", "last-picked");

            const switchElem = targetCell.querySelector(".track-link-switch");
            const hasSwitch = switchElem !== null;
            const clickedOnSwitch = e.target.closest(".track-link-switch") !== null;

            if (clickedOnSwitch || hasSwitch) {
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

    function highlightNextPosition(nextStepIndex, track, rowsA, rowsB) {
        const targetCell = getCellForStep(nextStepIndex, track, rowsA, rowsB);
        if (targetCell) {
            targetCell.classList.add("next_position");
        }
    }
});
