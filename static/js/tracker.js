// Probabbly I'll need to alter this later to make it better...

document.addEventListener("DOMContentLoaded", () => {
    const table = document.getElementById("tracker-table");
    if (!table) return;

    table.addEventListener("click", (e) => {
        if (e.target.tagName === 'A') return;

        const cell = e.target.closest("td.pick");
        if (!cell) return;

        document.querySelectorAll(".next_position").forEach(el => {
            el.classList.remove("next_position");
        });

        const columnIndex = cell.cellIndex;
        const currentRow = cell.parentElement;
        const rows = Array.from(table.querySelectorAll("tbody tr"));
        const startIdx = rows.indexOf(currentRow);

        if (startIdx === -1) return;

        for (let i = startIdx; i < Math.min(startIdx + 10, rows.length); i++) {
            const targetCell = rows[i].cells[columnIndex];
            if (targetCell) {
                targetCell.classList.add("next_position");
            }
        }
    });
});
