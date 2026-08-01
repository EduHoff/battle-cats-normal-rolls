document.addEventListener("DOMContentLoaded", () => {
    const eventSelect = document.getElementById("event");

    if (eventSelect) {
        eventSelect.addEventListener("change", () => {
            const rollSelects = document.querySelectorAll('select[name="rolls"]');
            rollSelects.forEach(select => {
                select.selectedIndex = 0;
            });

            eventSelect.form.submit();
        });
    }
});