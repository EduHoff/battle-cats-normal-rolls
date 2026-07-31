document.addEventListener("DOMContentLoaded", () => {
    const findSelect = document.getElementById("find-next");
    const trackerTable = document.getElementById("tracker-table");

    if (!findSelect || !trackerTable) return;

    function highlightSelectedUnit(unitName) {
        trackerTable.querySelectorAll(".highlight-find").forEach(cell => {
            cell.classList.remove("highlight-find");
        });

        if (!unitName || unitName.trim() === "") return;

        const pickCells = trackerTable.querySelectorAll("td.pick");

        pickCells.forEach(cell => {
            const mainLink = cell.querySelector("a");

            if (mainLink) {
                const cellText = mainLink.textContent.trim();

                if (cellText === unitName) {
                    cell.classList.add("highlight-find");
                }
            }
        });
    }

    highlightSelectedUnit(findSelect.value);

    findSelect.addEventListener("change", (e) => {
        highlightSelectedUnit(e.target.value);

        updateQueryParamInLinks("find", e.target.value);
    });

    function updateQueryParamInLinks(param, value) {
        const links = trackerTable.querySelectorAll("a");
        links.forEach(a => {
            try {
                const url = new URL(a.href);
                if (value) {
                    url.searchParams.set(param, value);
                } else {
                    url.searchParams.delete(param);
                }
                a.href = url.toString();
            } catch (err) {}
        });
    }
});
