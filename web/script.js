document.addEventListener('DOMContentLoaded', () => {
    const sidebarToggle = document.getElementById('sidebarToggle');
    const sidebar = document.querySelector('.sidebar');
    const wrapper = document.querySelector('.wrapper');

    if (sidebarToggle && sidebar && wrapper) {
        sidebarToggle.addEventListener('click', () => {
            sidebar.classList.toggle('collapsed');
            wrapper.classList.toggle('sidebar-open');
            document.querySelector('.main-content').classList.toggle('shifted');
        });
    }
});